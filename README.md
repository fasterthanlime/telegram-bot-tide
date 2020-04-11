# telegram-bot-tide

Using `tide` to receive Civ 6 webhooks + sending Telegram notifications
via `telegram-bot`.

## Running

```
cargo b
TELEGRAM_CHAT_ID=1 LISTEN_ADDR=0.0.0.0:6969 TELEGRAM_BOT_TOKEN=foobar ./target/debug/telegram-bot-tide
```

## Testing

```
curl -H "Content-Type: application/json" --request POST --data '{"value1":"test","value2":"lkl","value3":"42"}' http://localhost:6969
```
