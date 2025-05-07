# rsendmail

Yet another simple sendmail CLI.

## Usage

```
rsendmail [OPTIONS] --username <USERNAME> --password <PASSWORD> --receiver <RECEIVER> --server <SERVER> --subject <SUBJECT> --body <BODY>

Options:
  -u, --username <USERNAME>  authentication username
  -p, --password <PASSWORD>  authentication password
  -r, --receiver <RECEIVER>  receiver email address
  -s, --server <SERVER>      smtp server
  -P, --port <PORT>          smtp server port [default: 25]
  -t, --tls                  use tls or not
  -S, --subject <SUBJECT>    email subject
  -b, --body <BODY>          email body
  -e, --silent               silent mode
  -h, --help                 Print help
  -V, --version              Print version
```
