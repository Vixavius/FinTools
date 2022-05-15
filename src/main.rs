use std::error::Error;
use std::io;
use std::process;

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

    // Primary currency of the account, used for tracking ACB of secritys
    currency_base: Currency,

    // TODO: Currency vector?
    usd: &'a mut u64,
    cad: &'a mut u64,

    securities: &'a mut Vec<AccountSecurity<'a>>,
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

// Representation of an owned security including its historical activity
struct AccountSecurity<'a> {
    security: String,
    currency_base: Currency,

    // Value Tuples of open and closed security data
    // ie. costs or price of shares opened or closed
    open: &'a mut (Decimal, Decimal),
    closed: &'a mut Vec<(Decimal, Decimal, Decimal, Decimal)>,

    acb: &'a mut Decimal,
}

impl AccountSecurity<'_> {
    // Grab market value of open securities
    fn get_market_value(self, unit_price: Decimal) -> Decimal {
        return self.open.0 * unit_price;
    }

    // Grab book value of open securities
    fn get_book_value(self) -> Decimal {
        return self.open.0 * self.open.1;
    }

    // Open a local securities and update Adjusted Cost Basis
    fn open_security(
        self,
        quantity: Decimal,
        unit_price: Decimal,
        commission: Option<Decimal>,
    ) -> Decimal {
        /*  Adjusted Cost Basis for opening a security is:
            existing costs + new costs + commission
        ---------------------------------------------
              existing quantity + new quantity         */
        let units = self.open.0 + quantity;
        let new_costs =
            (unit_price * quantity) + commission.unwrap_or(Decimal::new(0, CURRENCY_PRECISION));
        let costs = self.open.1 + new_costs;

        self.open.0 = units;
        self.open.1 = costs;

        *self.acb = units / costs;

        // Return the amount to shift the balance by
        return new_costs;
    }

    // TODO: Open a foreign security for the account and update Adjusted Cost Basis, given the
    //       exchange rate of Foreign : Local currency
    fn open_security_foreign(
        self,
        quantity: Decimal,
        unit_price: Decimal,
        account_currency: Currency,
        secrity_currency: Currency,
        exchange_rate: Decimal,
    ) {
    }

    // TODO: Close a secrity given a quantity and average price
    fn close_security(
        self,
        quantity: Decimal,
        unit_price: Decimal,
        commission: Option<Decimal>,
    ) -> bool {
        if self.open.0 > quantity {}
        // Check given quantity of a secrity is open
        // If present, move it as closed with the given unit price and calculate the gain
        return false;
    }
}

fn example() -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::Reader::from_path("exampleinput/activities.csv")?;
    for result in rdr.records() {
        let record = result?;
        println!("{:?}", record);
    }
    Ok(())
}

fn parse_qt_record(record: csv::StringRecord) {
    // TODO: Parse a QT record
}

fn main() {
    if let Err(err) = example() {
        println!("error running example: {}", err);
        process::exit(1);
    }
}
