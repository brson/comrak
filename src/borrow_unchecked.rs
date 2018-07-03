use std::cell::RefCell;

pub trait BorrowUnchecked<T> {
    fn borrow_(&self) -> &T;
    fn borrow_mut_(&self) -> &mut T;
}

impl<T> BorrowUnchecked<T> for RefCell<T> {
    fn borrow_(&self) -> &T {
        unsafe { &*self.as_ptr() }
    }

    fn borrow_mut_(&self) -> &mut T {
        unsafe { &mut*self.as_ptr() }
    }
}
