mod front {
    pub mod react {
        pub fn learn_react() {}
        fn fuck_react() {}
    }

    pub mod vue {
        pub fn learn_vue() {}
        fn use_vue() {}
        fn fuck_vue() {}
    }
}

pub fn learn() {
    crate::front::react::learn_react();

    front::vue::learn_vue();
}
