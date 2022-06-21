use std::collections::HashMap;

pub enum VarKind {
    Static,
    Field,
    Arg,
    Var,
}

pub struct VarInfo {
    var_type: &'static str,
    kind: VarKind,
    index: usize,
}

struct VarCounter {
    count_static: usize,
    count_field: usize,
    count_arg: usize,
    count_var: usize,
}

impl VarCounter {
    fn new() -> Self {
        VarCounter { 0, 0, 0, 0 }
    }

    fn count_up(&mut self, kind: VarKind) {
        match kind {
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

    fn get_count(&self, kind: VarKind) -> usize {
        match kind {
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
    tbl_cls: HashMap<&str, VarInfo>,
    tbl_sub: HashMap<&str, VarInfo>,
    cnt_cls: VarCounter,
    cnt_sub: VarCounter,
}

impl SymbolTable {
    pub fn new() -> Self {
        SymbolTable {
            tbl_cls: HashMap::<&str, VarInfo>::new(),
            tbl_sub: HashMap::<&str, VarInfo>::new(),
            cnt_cls: VarCounter::new(),
            cnt_sub: VarCounter::new(),
        }
    }

    pub fn start_subroutine(&mut self) {
        self.tbl_sub.clear();
        self.cnt_sub.clear();
    }

    pub fn define(&mut self, name: &str, var_type: &str, kind: VarKind) {
        match kind {
            VarKind::Static | VarKind::Field => {
                tbl_cls.insert(name, VarInfo { name, var_type, cnt_cls.get_count(kind) });
                cnt_cls.count_up(kind);
            },
            VarKind::Arg | VarKind::Var => {
                tbl_sub.insert(name, VarInfo { name, var_type, cnt_sub.get_count(kind) });
                cnt_sub.count_up(kind);
            }
        }
    }

    pub fn var_count(&mut self, kind: VarKind) -> usize {
        match kind {
            VarKind::Static | VarKind::Field => {
                self.cnt_cls.get_count(kind)
            },
            VarKind::Arg | VarKind::Var => {
                self.cnt_sub.get_count(kind)
            }
        }
    }

    pub fn kind_of(name: &str) -> Option<VarKind> {
        match self.tbl_sub.get(name) {
            Some(i) => {
                Some(*i.kind)
            },
            None => {
                match self.tbl_cls.get(name) {
                    Some(j) => {
                        Some(*j.kind)
                    },
                    None => {
                        None
                    }
                }
            }
        }
    }

    pub fn type_of(name: &str) -> Option<&str> {
        match self.tbl_sub.get(name) {
            Some(i) => {
                Some(*i.var_type)
            },
            None => {
                match self.tbl_cls.get(name) {
                    Some(j) => {
                        Some(*j.var_type)
                    },
                    None => {
                        None
                    }
                }
            }
        }
    }

    pub fn index_of(name: &str) -> Option<usize> {
        match self.tbl_sub.get(name) {
            Some(i) => {
                Some(*i.index)
            },
            None => {
                match self.tbl_cls.get(name) {
                    Some(j) => {
                        Some(*j.index)
                    },
                    None => {
                        None
                    }
                }
            }
        }
    }
}