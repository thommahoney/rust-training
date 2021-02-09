use std::ptr::NonNull;
use leveldb_sys::*;
use std::ptr::null_mut;
use std::ffi::CString;
use libc::{size_t,c_void};

#[derive(Debug,Eq,PartialEq)]
pub enum DatabaseError {
    IDunno
}

#[derive(Debug,Eq,PartialEq)]
pub struct Database {
    ptr: NonNull<leveldb_t>,
}

impl Database {
    pub fn new(options: Options, name: &str) -> Result<Self, DatabaseError> {
        let name = CString::new(name).unwrap();

        let mut err_ptr = null_mut();
        let ldb = unsafe { leveldb_open(options.ptr.as_ptr(), name.as_ptr(), &mut err_ptr) };

        if err_ptr != null_mut() {
           return Err(DatabaseError::IDunno); 
        }

        Ok(Database { ptr: NonNull::new(ldb).unwrap() })
    }

    pub fn close(&mut self) -> Result<(), DatabaseError> {
        todo!()
    }

    pub fn get(&self, key: &str) -> Result<Option<String>, DatabaseError> {
        let read_options = unsafe { leveldb_readoptions_create() };
        let err_ptr = null_mut();
        let mut val_length: size_t = 0;

        let data = unsafe {
            leveldb_get(
                self.ptr.as_ptr(),
                read_options,
                key.as_ptr() as *const _,
                key.len(),
                &mut val_length,
                err_ptr, 
            )
        };

        if err_ptr == null_mut() {
            if data == null_mut() {
                Ok(None)
            } else {
                let slice = unsafe { std::slice::from_raw_parts(data as *mut u8, val_length) };
                let result = String::from_utf8(slice.to_vec()).unwrap();

                unsafe { leveldb_free(data as *mut c_void) };

                Ok(Some(result))
            }
        } else {
            unsafe { leveldb_free(*err_ptr as *mut c_void) };

            Err(DatabaseError::IDunno)
        }
    }

    pub fn put(&self, key: &str, value: &str) -> Result<(), DatabaseError> {
        let write_options = unsafe { leveldb_writeoptions_create() };
        let err_ptr = null_mut();

        unsafe {
            leveldb_put(
                self.ptr.as_ptr(),
                write_options, 
                key.as_ptr() as *const _, 
                key.len(), 
                value.as_ptr() as *const _, 
                value.len(),
                err_ptr,
            );
        };

        if err_ptr == null_mut() {
            Ok(())
        } else {
            unsafe { leveldb_free(*err_ptr as *mut c_void) };

            Err(DatabaseError::IDunno)
        }
    }
}

pub struct Options {
    ptr: NonNull<leveldb_options_t>
}

impl Options {
    pub fn new(create_if_missing: bool) -> Self {
        let leveldb_opt = unsafe { leveldb_options_create() };
        unsafe { leveldb_options_set_create_if_missing(leveldb_opt, u8::from(create_if_missing)) };

        Options {
            ptr: NonNull::new(leveldb_opt).unwrap(),
        }
    }

    pub fn destroy(&mut self) -> Result<(), DatabaseError> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn it_works() {
        let options = Options::new(true);
        let db = Database::new(options, "database");

        assert!(db.is_ok());
    }

    #[test]
    fn it_breaks() {
        let options = Options::new(false);
        let db = Database::new(options, "does-not-exist");

        assert_eq!(db, Err(DatabaseError::IDunno));
    }

    #[test]
    fn it_gets() {
        let options = Options::new(true);
        let db = Database::new(options, "gets-db").unwrap();
        let result = db.get("caslyn");

        assert_eq!(result, Ok(None));
    }

    #[test]
    fn it_puts() {
        let options = Options::new(true);
        let db = Database::new(options, "puts-db").unwrap();
        let result = db.put("caslyn", "tonelli");

        assert!(result.is_ok());
    }
}
