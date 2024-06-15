## A simple terminal currency convertor
**How to run**
```
currency_convetor amount starting_currency_code currency_to_convert_to (optional: -r)
```
Example:
```
$ currency_convertor 500 USD GBP
500 USD is 394.5705 GBP
```
It uses the [OER](openexchangerates.org) API with a user provided API key.
The first time you run it, you must refresh your currencies with the -r flag. After that, you can do so as needed.
Works on Windows, Linux, and Mac.