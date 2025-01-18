## A minimalist terminal currency converter
**How to run**
```
oer_conversion_rates amount starting_currency_code currency_to_convert_to (optional: -r,-k)
```
Example:
```
$ oer_conversion_rates 500 USD GBP
500 USD is 410.61 GBP
```
It uses the [OER](openexchangerates.org) API with a user provided API key.
The first time you run it, you add an OER API key with the -k flag. After that, you can do so as needed.
If you want to refresh your currencies, use the -r flag. 
Works on Windows, Linux, and Mac.
