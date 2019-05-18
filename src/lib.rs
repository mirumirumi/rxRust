
pub mod subject;
pub mod ops;
mod subscription;

pub use subject::Subject;
pub use subscription::Subscription;

pub trait Observable<'a>: Sized {
  /// The type of the elements being emitted.
  type Item: Sized;
  // the Subscription subsribe method return. 
  type Unsubcribe;

  fn subscribe<O>(self, observer: O) -> Self::Unsubcribe
  where
    O: 'a + FnMut(Self::Item);


  fn broadcast(self) -> Subject<'a, Self::Item>
  where
    Self: 'a,
  {
    Subject::from_stream(self)
  }
}

pub trait Observer {
  type Item;

  fn next(&self, v: Self::Item) -> &Self;
}