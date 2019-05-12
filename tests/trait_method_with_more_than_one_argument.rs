use mockiato::mockable;

#[mockable]
trait MessageSender {
    fn send_message(&self, recipient: &str, message: &str);
}

#[test]
fn test() {
    let mut message_sender = MessageSenderMock::new();

    message_sender
        .expect_send_message(
            |f| f.partial_eq("Jane"),
            |f| f.partial_eq("Don't make lemonade"),
        )
        .times(1..);

    message_sender.send_message("Jane", "Don't make lemonade")
}
