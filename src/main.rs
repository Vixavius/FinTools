use rust_decimal::Decimal;

const CURRENCY_PRECISION: u32 = 8;

// Future Activities:
//   - Return of Capital
//   - Capital Gains Dividend
//   - Reinvested Dividend
//   - Reinvested Capital Gains Distribution
//   - Split

enum Activity {
    DEPOSIT,
    WITHDRAWAL,
    BUY,
    SELL,
    DIVIDEND,
    FEE,
    UNKNOWN,
}

enum Currency {
    CAD,
    USD,
}

// Representation of a brokerage account and historical activity
struct Account<'a> {
    id: u32,
    taxable: bool,

    // Primary currency of the account, used for tracking ACB of symbols
    currency_base: Currency,

    // TODO: Currency vector?
    usd: &'a mut u64,
    cad: &'a mut u64,

    symbols: &'a mut Vec<AccountSymbol<'a>>,
}

impl Account<'_> {
    // Increment value of CAD as given in cents
    fn deposit_cad(self, units: u64) {
        *self.cad += units;
    }

    fn deposit_usd(self, units: u64) {
        *self.usd += units;
    }

    fn withdraw_cad(self, units: u64) -> bool {
        if *self.cad >= units {
            *self.cad -= units;
            return true;
        }

        return false;
    }

    fn withdraw_usd(self, units: u64) -> bool {
        if *self.usd >= units {
            *self.usd -= units;
            return true;
        }

        return false;
    }
}

// Representation of an owned symbol including its historical activity
struct AccountSymbol<'a> {
    symbol: String,
    currency_base: Currency,

    // Value Tuples of open and closed symbol data
    // ie. costs or price of shares opened or closed
    open: &'a mut (i64, Decimal),
    closed: &'a mut Vec<(i64, Decimal, Decimal, Decimal)>,

    acb: &'a mut Decimal,
}

impl AccountSymbol<'_> {
    // Grab market value of open symbols
    fn get_market_value(self, unit_price: Decimal) -> Decimal {
        let mut sum = Decimal::new(0, CURRENCY_PRECISION);
        let unit_decimal: Decimal = self.open.0.into();

        sum += unit_decimal * unit_price;

        return sum;
    }

    // Grab book value of open symbols
    fn get_book_value(self) -> Decimal {
        let mut sum: Decimal = Decimal::new(0, CURRENCY_PRECISION);
        let unit_decimal: Decimal = self.open.0.into();

        sum += unit_decimal * self.open.1;

        return sum;
    }

    // Open a local symbol for the account and update Adjusted Cost Basis
    fn open_symbol(self, quantity: i64, unit_price: Decimal, commission: Option<Decimal>) {
        // Adjusted Cost Basis for opening a symbol is:
        //
        //            existing costs + new costs + commission
        //        ---------------------------------------------
        //              existing quantity + new quantity
        //

        let units = self.open.0 + quantity;

        // TODO: Reduce balance of the correct currency

        let costs = self.open.1
            + (unit_price * Decimal::new(quantity, CURRENCY_PRECISION))
            + commission.unwrap_or(Decimal::new(0, CURRENCY_PRECISION));

        self.open.0 = units;
        self.open.1 = costs;

        let unit_decimal = Decimal::new(units, CURRENCY_PRECISION);

        *self.acb = unit_decimal / costs;
    }

    // TODO: Open a foreign symbol for the account and update Adjusted Cost Basis, given the
    //       exchange rate of Foreign : Local currency
    fn open_symbol_foreign(
        self,
        quantity: u32,
        unit_price: Decimal,
        account_currency: Currency,
        symbol_currency: Currency,
        exchange_rate: Decimal,
    ) {
    }

    // TODO: Close a symbol given a quantity and average price
    fn close_symbol(quantity: u32, unit_price: Decimal) -> bool {
        // Check given quantity of a symbol is open
        // If present, move it as closed with the given unit price and calculate the gain
        return false;
    }
}

fn main() {}
