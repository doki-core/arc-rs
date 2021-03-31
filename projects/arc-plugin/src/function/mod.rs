
use std::collections::HashMap;
use std::fmt::{Display, Formatter, Debug};
use std::{fmt, io};
use libloading::Library;
use std::rc::Rc;
use std::ffi::OsStr;

#[derive(Debug, Clone, PartialEq)]
pub enum RkError {
    InvalidArgumentCount { expected: usize, found: usize },
    Other { msg: String },
}

#[derive(Debug)]
pub enum Value {
    Null
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Debug::fmt(self, f)
    }
}


impl<S: ToString> From<S> for RkError {
    fn from(other: S) -> RkError {
        RkError::Other {
            msg: other.to_string(),
        }
    }
}


pub static RUSTC_VERSION: &str = env!("RUSTC_VERSION");

pub trait Function {
    fn call(&self, args: &[Value], kvs: &HashMap<String, Value>) -> Result<Value, RkError>;
}

#[derive(Copy, Clone)]
pub struct PluginDeclaration {
    pub rust_version: &'static str,
    pub register: unsafe extern "C" fn(&mut dyn Register),
}

pub trait Register {
    fn register_function(&mut self, name: &str, function: Box<dyn Function>);
}


/// A map of all externally provided functions.
#[derive(Default)]
pub struct ExternalFunctions {
    functions: HashMap<String, Box<dyn Function>>,
    libraries: Vec<Rc<Library>>,
}

pub struct FunctionRegistrar {
    lib: Rc<Library>,
    functions: HashMap<String, Box<dyn Function>>,
}

impl ExternalFunctions {
    pub fn new() -> ExternalFunctions { ExternalFunctions::default() }

    pub fn call(
        &self,
        function: &str,
        args: &[Value],
        kvs: &HashMap<String, Value>,
    ) -> Result<Value, RkError> {
        self.functions
            .get(function)
            .ok_or_else(|| format!("\"{}\" not found", function))?
            .call(args, kvs)
    }

    /// Load a plugin library and add all contained functions to the internal
    /// function table.
    ///
    /// # Safety
    ///
    /// A plugin library **must** be implemented using the
    /// [`plugins_core::plugin_declaration!()`] macro. Trying manually implement
    /// a plugin without going through that macro will result in undefined
    /// behaviour.
    pub unsafe fn load<P: AsRef<OsStr>>(
        &mut self,
        library_path: P,
    ) -> io::Result<()> {
        // load the library into memory
        let library = Rc::new(Library::new(library_path)?);

        // get a pointer to the plugin_declaration symbol.
        let decl = library
            .get::<*mut PluginDeclaration>(b"plugin_declaration\0")?
            .read();

        // version checks to prevent accidental ABI incompatibilities
        if decl.rust_version != crate::RUSTC_VERSION {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Version mismatch",
            ));
        }

        let mut registrar = FunctionRegistrar::new(Rc::clone(&library));

        (decl.register)(&mut registrar);

        // add all loaded plugins to the functions map
        self.functions.extend(registrar.functions);
        // and make sure ExternalFunctions keeps a reference to the library
        self.libraries.push(library);

        Ok(())
    }
}


impl FunctionRegistrar {
    fn new(lib: Rc<Library>) -> FunctionRegistrar {
        FunctionRegistrar { lib, functions: HashMap::default() }
    }
}

impl Register for FunctionRegistrar {
    fn register_function(&mut self, name: &str, function: Box<dyn Function>) {
        self.functions.insert(name.to_string(), function);
    }
}


#[macro_export]
macro_rules! export_plugin {
    ($register:expr) => {
        #[doc(hidden)]
        #[no_mangle]
        pub static plugin_declaration: $crate::PluginDeclaration =
            $crate::PluginDeclaration {
                rust_version: $crate::RUSTC_VERSION,
                register: $register,
            };
    };
}
