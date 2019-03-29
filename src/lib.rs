
use serenity::model::user::User;


pub fn is_phil(user: &User) -> bool {
    user.name.to_lowercase().contains("phil")
}

pub fn is_perfect(user: &User) -> bool {
    user.name.to_lowercase().contains("perfectmak")
}

pub fn is_close_message(message: &str) -> bool {
    let trigger_message = vec![
        "kill",
        "close",
        "remove",
        "delete",
        "shutdown"
    ];

    trigger_message.iter()
        .map(|w| message.contains(w))
        .fold(false, |acc, b| acc || b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_close_message_is_true() {
        assert_eq!(
            true,
            is_close_message("just closed a bunch of issues that I think were stale")
        );
    }

    #[test]
    fn is_close_message_is_false() {
        assert_eq!(
            false,
            is_close_message("that would be pretty wonderful")
        );
    }
}