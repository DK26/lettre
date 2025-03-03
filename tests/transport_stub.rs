#[cfg(test)]
#[cfg(feature = "builder")]
mod sync {
    use lettre::{transport::stub::StubTransport, Message, Transport};

    #[test]
    fn stub_transport() {
        let sender_ok = StubTransport::new_ok();
        let sender_ko = StubTransport::new_error();
        let email = Message::builder()
            .from("NoBody <nobody@domain.tld>".parse().unwrap())
            .reply_to("Yuin <yuin@domain.tld>".parse().unwrap())
            .to("Hei <hei@domain.tld>".parse().unwrap())
            .subject("Happy new year")
            .body(String::from("Be happy!"))
            .unwrap();

        sender_ok.send(&email).unwrap();
        sender_ko.send(&email).unwrap_err();

        let expected_messages = vec![(
            email.envelope().clone(),
            String::from_utf8(email.formatted()).unwrap(),
        )];
        assert_eq!(sender_ok.messages(), expected_messages);
    }
}

#[cfg(test)]
#[cfg(all(feature = "builder", feature = "tokio1"))]
mod tokio_1 {
    use lettre::{transport::stub::AsyncStubTransport, AsyncTransport, Message};
    use tokio1_crate as tokio;

    #[tokio::test]
    async fn stub_transport_tokio1() {
        let sender_ok = AsyncStubTransport::new_ok();
        let sender_ko = AsyncStubTransport::new_error();
        let email = Message::builder()
            .from("NoBody <nobody@domain.tld>".parse().unwrap())
            .reply_to("Yuin <yuin@domain.tld>".parse().unwrap())
            .to("Hei <hei@domain.tld>".parse().unwrap())
            .subject("Happy new year")
            .body(String::from("Be happy!"))
            .unwrap();

        sender_ok.send(email.clone()).await.unwrap();
        sender_ko.send(email.clone()).await.unwrap_err();

        let expected_messages = vec![(
            email.envelope().clone(),
            String::from_utf8(email.formatted()).unwrap(),
        )];
        assert_eq!(sender_ok.messages().await, expected_messages);
    }
}

#[cfg(test)]
#[cfg(all(feature = "builder", feature = "async-std1"))]
mod asyncstd_1 {
    use lettre::{transport::stub::AsyncStubTransport, AsyncTransport, Message};

    #[async_std::test]
    async fn stub_transport_asyncstd1() {
        let sender_ok = AsyncStubTransport::new_ok();
        let sender_ko = AsyncStubTransport::new_error();
        let email = Message::builder()
            .from("NoBody <nobody@domain.tld>".parse().unwrap())
            .reply_to("Yuin <yuin@domain.tld>".parse().unwrap())
            .to("Hei <hei@domain.tld>".parse().unwrap())
            .subject("Happy new year")
            .body(String::from("Be happy!"))
            .unwrap();

        sender_ok.send(email.clone()).await.unwrap();
        sender_ko.send(email.clone()).await.unwrap_err();

        let expected_messages = vec![(
            email.envelope().clone(),
            String::from_utf8(email.formatted()).unwrap(),
        )];
        assert_eq!(sender_ok.messages().await, expected_messages);
    }
}
