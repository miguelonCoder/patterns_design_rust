

pub trait Command {
    fn execute(&mut self );
    fn rollback(&mut self );
}