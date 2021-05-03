pub struct EventEmitter<T, F: FnOnce(T) -> ()> {
    subscribers: Vec<F>,
}

impl<T, F> EventEmitter<T, F> {
    pub fn subscribe() {

    }

    pub fn publish() {

    }

    pub fn unsubscribe() {

    }
}
