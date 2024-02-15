mod parent;

pub use parent::ParentOperate;

pub trait OperateTrait {
    type Insert;
    type Update;
    type Delete;
    type Retrieve;

    fn insert(self) -> Self::Insert;
    fn update(self) -> Self::Update;
    fn delete(self) -> Self::Delete;
    fn retrieve(self) -> Self::Retrieve;
}

#[macro_export]
macro_rules! db_operate {
    ($id:ident) => {
        pub struct $id;

        pub struct Insert;
        pub struct Update;
        pub struct Delete;
        pub struct Retrieve;

        impl $crate::operations::OperateTrait for $id {
            type Insert = Insert;
            type Update = Update;
            type Delete = Delete;
            type Retrieve = Retrieve;

            fn insert(self) -> Self::Insert {
                Insert
            }
            fn update(self) -> Self::Update {
                Update
            }
            fn delete(self) -> Self::Delete {
                Delete
            }
            fn retrieve(self) -> Self::Retrieve {
                Retrieve
            }
        }
    };
}
