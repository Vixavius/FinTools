# FinTools
TODO:
- Portfolio overview of investment accounts
- Adjusted Cost Basis calculator for taxable investment accounts

Transaction Format Examples
|       Transaction Date |        Settlement Date | Action | Symbol  | Description             | Quantity |       Price | Gross Amount | Commission | Net Amount | Currency | Account # | Activity Type | Account Type      |
| ---------------------: | ---------------------: | :----- | :------ | :---------------------- | -------: | ----------: | -----------: | ---------: | ---------: | :------- | :-------- | :------------ | :---------------- |
| 2020-01-01 12:00:00 AM | 2020-01-01 12:00:00 AM | CON    |         | CON 12340001            |  0.00000 |  0.00000000 |         0.00 |       0.00 |   10000.00 | CAD      | 12340001  | Deposits      | Individual margin |
| 2020-01-02 12:00:00 AM | 2020-01-02 12:00:00 AM | Buy    | VEQT.TO | Vanguard All-Equity ETF | 29.00000 | 34.89000000 |     -1011.81 |      -0.10 |   -1011.91 | CAD      | 12340001  | Trades        | Individual margin |
| 2020-01-02 12:00:00 AM | 2020-01-02 12:00:00 AM | Buy    | VEQT.TO | Vanguard All-Equity ETF | 32.00000 | 34.90000000 |     -1116.80 |      -0.10 |   -1116.90 | CAD      | 12340001  | Trades        | Individual margin |
| 2020-01-03 12:00:00 AM | 2020-01-03 12:00:00 AM | DIV    | VEQT.TO | Vanguard All-Equity ETF |  0.00000 |  0.51000000 |         0.00 |       0.00 |     206.18 | CAD      | 12340001  | Dividends     | Individual margin |
| 2020-01-04 12:00:00 AM | 2020-01-04 12:00:00 AM | Sell   | VEQT.TO | Vanguard All-Equity ETF |  5.00000 | 36.01000000 |       180.05 |      -5.01 |     175.04 | CAD      | 12340001  | Trades        | Individual margin |

From the Above, the Account 12340001 features:
* A balance of $8,252.41 CAD
* A balance of 56 VEQT symbols
* An Adjusted Cost Basis of $34.90 for the VEQT.TO security
* A capital gain of $206.18 from the dividend, with a net gain of $62.69 including the capital loss from selling