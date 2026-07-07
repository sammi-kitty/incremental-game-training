# incremental-game-training
Coding practice consisting of an incremental game, written in Rust and Python

## TODO
- [ ] json-based templates of machines
- [ ] json-based save system
- [ ] ticking system (to be thought out)
- [ ] menu system (independent of ticking system, to be thought out)
    - [ ] shop
#### MAYBE
- [ ] graphics

## Project Structure

### Data management

#### json-based templates
Each "machine" should be built from a json template, specifying:
- **Machine Template**
- Name (string)
- Description (optional, string)
- Cost (number)
- Base production (number)
- Upgrades (array of upgrade templates)
    - **Upgrade Template**
    - Name (string)
    - Description (optional, string)
    - Cost (number)
    - Multiplier (number)
    - Array of required upgrade names (array of strings)

#### json-based saves
A save should consist of:
- **Save Object**
- Name (string)
- Time spent (translate from ticks to seconds/hours/days, number)
- Machines (array of Machine Objects)
    - **Machine Object**
    - Name (string)
    - Description (optional, string)
    - Cost (number)
    - Base production (number)
    - *Amount owned* (number)
    - Upgrades (array of Upgrade Objects)
        - **Upgrade Object**
        - Name (string)
        - Description (optional, string)
        - Cost (number)
        - Multiplier (number)
        - Array of required upgrade names (array of strings)
        - *is_purchased* (bool)

#### Notes
There needs to be a way to handle new upgrades being added (in an update), and them automatically being added to the lists of upgrades in existing save files