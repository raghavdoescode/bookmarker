use serenity::{async_trait, model::{channel::Message, gateway::Ready, channel::Reaction}, prelude::*};
use std::env;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!ping" {
            msg.reply(&ctx, "Pong!").await.unwrap();
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }

    async fn reaction_add(&self, _ctx: Context, reaction: Reaction) {
        if reaction.emoji.unicode_eq("🔖") {
            let message = reaction.message(&_ctx).await.unwrap();
            let user = reaction.user(&_ctx).await.unwrap();

            if message.attachments.len() == 0 {
                user.direct_message(&_ctx, |m| {
                    m.content(format!("You bookmarked a message: {}", message.content))
                }).await.unwrap();

            } else {
                // Handle attachments
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let token = env::var("TOKEN").expect("unable to get token");
    let mut client = Client::builder(&token)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
