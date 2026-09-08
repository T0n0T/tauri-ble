use tokio::sync::mpsc;

#[allow(dead_code)]
pub(crate) fn send_channel_data<T>(tx: &mpsc::Sender<T>, data: T) -> bool {
    tx.blocking_send(data).is_ok()
}

#[cfg(test)]
mod tests {
    use super::send_channel_data;
    use tokio::sync::mpsc;

    #[test]
    fn closed_receiver_does_not_panic() {
        let (tx, rx) = mpsc::channel(1);
        drop(rx);

        assert!(!send_channel_data(&tx, 1));
    }
}
