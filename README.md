## A minimalist terminal currency converter
**How to run**
```
currency_converter amount starting_currency_code currency_to_convert_to (optional: -r,-k)
```
Example:
```
$ currency_converter 500 USD GBP
500 USD is 394.5705 GBP
```
It uses the [OER](openexchangerates.org) API with a user provided API key.
The first time you run it, you add an OER API key with the -k flag. After that, you can do so as needed.
If you want to refresh your currencies, use the -r flag. 
Works on Windows, Linux, and Mac.
