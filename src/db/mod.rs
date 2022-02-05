pub mod inputs;
pub mod outputs;
pub mod scripts;
pub mod tags;

trait Crud {
    // todo!
    type Error;
    type Success;
    type DbConnection;

    fn create(self, conn: Self::DbConnection) -> Result<Self::Success, Self::Error>;

    fn read(self, conn: Self::DbConnection) -> Result<Self::Success, Self::Error>;

    fn update(self, conn: Self::DbConnection) -> Result<Self::Success, Self::Error>;

    fn delete(self, conn: Self::DbConnection) -> Option<Self::Error>;
}
