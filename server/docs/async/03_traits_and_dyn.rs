




























trait MessageSender {
    fn send(&self, message: &str);
}








struct EmailSender {
    smtp_server: String,
}


impl MessageSender for EmailSender {
    fn send(&self, message: &str) {
        println!(
            "Sending email via {}: {}",
            self.smtp_server, message
        );
    }
}

struct SlackSender {
    channel: String,
}


impl MessageSender for SlackSender {
    fn send(&self, message: &str) {
        println!(
            "Posting to Slack channel {}: {}",
            self.channel, message
        );
    }
}











fn notify_static(sender: &impl MessageSender, msg: &str) {
    sender.send(msg);
}

















fn notify_dynamic(sender: &dyn MessageSender, msg: &str) {
    sender.send(msg);
    
    
    
}

































use std::sync::Arc;

fn main() {
    
    let email = EmailSender {
        smtp_server: String::from("smtp.gmail.com"),
    };
    let slack = SlackSender {
        channel: String::from("#alerts"),
    };

    
    notify_static(&email, "Static email");
    notify_static(&slack, "Static slack");

    
    notify_dynamic(&email, "Dynamic email");
    notify_dynamic(&slack, "Dynamic slack");

    
    
    let senders: Vec<Arc<dyn MessageSender>> = vec![
        Arc::new(EmailSender {
            smtp_server: String::from("smtp.company.com"),
        }),
        Arc::new(SlackSender {
            channel: String::from("#general"),
        }),
    ];

    
    
    for sender in &senders {
        sender.send("Hello from the loop!");
    }

    
    
    

    
    let repo: Arc<dyn MessageSender> = Arc::new(EmailSender {
        smtp_server: String::from("smtp.crm.com"),
    });
    
    
    
    
    

    
    repo.send("This works without knowing the concrete type!");

    
    
    let test_repo: Arc<dyn MessageSender> = Arc::new(SlackSender {
        channel: String::from("#test"),
    });
    test_repo.send("Now using Slack instead — same code path!");
}










































