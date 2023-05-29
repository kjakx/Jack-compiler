use std::collections::HashMap;
use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub enum VarType {
    Int,
    Char,
    Boolean,
    ClassName(String),
}

impl fmt::Display for VarType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VarType::Int       => write!(f, "int"),
            VarType::Char      => write!(f, "char"), 
            VarType::Boolean   => write!(f, "bool"),
            VarType::ClassName(class_name) => write!(f, "{}", &class_name)
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum VarKind {
    Static,
    Field,
    Arg,
    Var,
}

impl fmt::Display for VarKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VarKind::Static => write!(f, "static"),
            VarKind::Field  => write!(f, "field"), 
            VarKind::Arg    => write!(f, "arg"),
            VarKind::Var    => write!(f, "var")
        }
    }
}

pub struct VarInfo {
    var_type: VarType,
    kind: VarKind,
    index: usize,
}

impl VarInfo {
    fn new(var_type: VarType, kind: VarKind, index: usize) -> Self {
        VarInfo {
            var_type,
            kind,
            index
        }
    }
}

struct VarCounter {
    count_static: usize,
    count_field: usize,
    count_arg: usize,
    count_var: usize,
}

impl VarCounter {
    fn new() -> Self {
        VarCounter {
            count_static: 0,
            count_field : 0,
            count_arg   : 0,
            count_var   : 0
        }
    }

    fn count_up(&mut self, var_kind: VarKind) {
        match var_kind {
            VarKind::Static => {
                self.count_static += 1;
            },
            VarKind::Field => {
                self.count_field += 1;
            },
            VarKind::Arg => {
                self.count_arg += 1;
            },
            VarKind::Var => {
                self.count_var += 1;
            }
        }
    }

    fn get_count(&self, var_kind: VarKind) -> usize {
        match var_kind {
            VarKind::Static => {
                self.count_static
            },
            VarKind::Field => {
                self.count_field
            },
            VarKind::Arg => {
                self.count_arg
            },
            VarKind::Var => {
                self.count_var
            }
        }
    }

    fn clear(&mut self) {
        self.count_static = 0;
        self.count_field = 0;
        self.count_arg = 0;
        self.count_var = 0;
    }
}

pub struct SymbolTable {
    tbl_cls: HashMap<String, VarInfo>,
    tbl_sub: HashMap<String, VarInfo>,
    cnt_cls: VarCounter,
    cnt_sub: VarCounter,
}

impl SymbolTable {
    pub fn new() -> Self {
        SymbolTable {
            tbl_cls: HashMap::<String, VarInfo>::new(),
            tbl_sub: HashMap::<String, VarInfo>::new(),
            cnt_cls: VarCounter::new(),
            cnt_sub: VarCounter::new(),
        }
    }

    pub fn start_subroutine(&mut self) {
        self.tbl_sub.clear();
        self.cnt_sub.clear();
    }

    pub fn define(&mut self, name: &str, var_kind: VarKind, var_type: VarType,) {
        match var_kind {
            VarKind::Static | VarKind::Field => {
                self.tbl_cls.insert(name.into(), VarInfo::new(var_type, var_kind, self.cnt_cls.get_count(var_kind)));
                self.cnt_cls.count_up(var_kind);
            },
            VarKind::Arg | VarKind::Var => {
                self.tbl_sub.insert(name.into(), VarInfo::new(var_type, var_kind, self.cnt_sub.get_count(var_kind)));
                self.cnt_sub.count_up(var_kind);
            }
        }
    }

    pub fn contains(&mut self, key: &str) -> bool {
        self.tbl_sub.contains_key(key) | self.tbl_cls.contains_key(key)
    }

    pub fn var_count(&mut self, var_kind: VarKind) -> usize {
        match var_kind {
            VarKind::Static | VarKind::Field => {
                self.cnt_cls.get_count(var_kind)
            },
            VarKind::Arg | VarKind::Var => {
                self.cnt_sub.get_count(var_kind)
            }
        }
    }

    pub fn kind_of(&self, name: &str) -> Option<&VarKind> {
        match self.tbl_sub.get(name.into()) {
            Some(i) => {
                Some(&i.kind)
            },
            None => {
                match self.tbl_cls.get(name.into()) {
                    Some(j) => {
                        Some(&j.kind)
                    },
                    None => {
                        None
                    }
                }
            }
        }
    }

    pub fn type_of(&self, name: &str) -> Option<&VarType> {
        match self.tbl_sub.get(name.into()) {
            Some(i) => {
                Some(&i.var_type)
            },
            None => {
                match self.tbl_cls.get(name.into()) {
                    Some(j) => {
                        Some(&j.var_type)
                    },
                    None => {
                        None
                    }
                }
            }
        }
    }

    pub fn index_of(&self, name: &str) -> Option<&usize> {
        match self.tbl_sub.get(name.into()) {
            Some(i) => {
                Some(&i.index)
            },
            None => {
                match self.tbl_cls.get(name.into()) {
                    Some(j) => {
                        Some(&j.index)
                    },
                    None => {
                        None
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new_symbol_table() {
        let mut test = SymbolTable::new();
        assert_eq!(test.var_count(VarKind::Static), 0);
        assert_eq!(test.var_count(VarKind::Field), 0);
        assert_eq!(test.var_count(VarKind::Arg), 0);
        assert_eq!(test.var_count(VarKind::Var), 0);
        assert_eq!(test.kind_of("test"), None);
        assert_eq!(test.type_of("test"), None);
        assert_eq!(test.index_of("test"), None);
    }

    #[test]
    fn test_define_symbols() {
        let mut test = SymbolTable::new();
        test.define("test1", VarKind::Var, VarType::Boolean);
        test.define("test2", VarKind::Arg, VarType::Int);
        test.define("test3", VarKind::Static, VarType::Char);
        test.define("test4", VarKind::Field, VarType::Int);
        test.define("test5", VarKind::Static, VarType::ClassName(String::from("TestClass")));
        assert_eq!(test.kind_of("test1"), Some(&VarKind::Var));
        assert_eq!(test.type_of("test2"), Some(&VarType::Int));
        assert_eq!(test.index_of("test5"), Some(&1));
        assert_eq!(test.var_count(VarKind::Static), 2);
        assert_eq!(test.kind_of("ghost"), None);
    }

    #[test]
    fn test_start_subroutine() {
        let mut test = SymbolTable::new();
        test.define("test1", VarKind::Var, VarType::Boolean);
        test.define("test2", VarKind::Arg, VarType::Int);
        test.define("test3", VarKind::Static, VarType::Char);
        test.define("test4", VarKind::Field, VarType::Int);
        test.define("test5", VarKind::Static, VarType::ClassName(String::from("TestClass")));
        test.start_subroutine();
        assert_eq!(test.kind_of("test1"), None);
        assert_eq!(test.type_of("test2"), None);
        assert_eq!(test.index_of("test5"), Some(&1));
        assert_eq!(test.var_count(VarKind::Static), 2);
        assert_eq!(test.var_count(VarKind::Field), 1);
        assert_eq!(test.var_count(VarKind::Var), 0);
        assert_eq!(test.var_count(VarKind::Arg), 0);
        assert_eq!(test.kind_of("ghost"), None);
    }
}