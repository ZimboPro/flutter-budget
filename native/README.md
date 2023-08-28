# Native Layer

Native laywer is written in Rust

## Requirements

 - [ ] Create database
 - [ ] Logic to migrate database if required
 - [ ] Read SQL directly from file?
 - [ ] Able to see history
 - [ ] Create new Envelopes if old one is editted and has transactions already from a previous month
 - [ ] Determine if Envelope is still in-use/active
 - [ ] Unit tests
 - [ ] Current state of envelope
    - [ ] Current amount spent in envelope
 - [ ] Update state if transaction is recorded?
 - [ ] CRUD for transactions
    - [ ] If transaction is edited to occur in a different month
        - [ ] Create new transaction based on transaction
        - [ ] Delete original transaction

## Database

 * Envelope Table
    * ID - Integer
    * Name - String
    * Description - String
    * Allocated Amount - Real
    * Active - Boolean

 * Transaction Table
    * ID - Integer
    * Envelope ID - Integer
    * Date - Datetime
    * Name - String
    * Description - String (Optional)
    * Amount - Real

 * Active Envelopes
