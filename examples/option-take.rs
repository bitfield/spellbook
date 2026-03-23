fn main() {
    struct Thing(Option<String>);

    impl Thing {
        fn ping(&mut self) {
            let ring = self.0.take();
            println!("{ring:?}",);
        }
    }

    let mut ding = Thing(Some("tingaling".into()));
    ding.ping();
}
