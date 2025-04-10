# Change Log

Notable changes to the "Ethnopians" crate will be documented in this file.

Check [Keep a Changelog](http://keepachangelog.com/) for recommendations on how to structure this file.

## [0.3.0] - 2025-04-10

### Changed

- de Bevyfied it

### Removed

- I moved the Bevy system into the bevy front end create


## [0.2.5] - 2025-03-31

### Added

- From<Option<T:Into(StackItem)>> for StackItem

- From<*Option<T:Into(StackItem + Clone)>> for StackItem

- From<Entity> for StackItem

- From<&Entity> for StackItem


## [0.2.4] - 2025-03-28

### Changed

- replaced floats with orderedF32

### Added

- 'Debug, Clone, PartialEq, Eq, PartialOrd, Ord' to everything

## [0.2.3] - 2025-03-25

### Changed

- made StackItem:Table COWs

## [0.2.2] - 2025-03-25

### Change 

- replaced RefCell with RwLock

## [0.2.1] - 2025-03-25

### Added

- Instructions needed for tha HermitAI

### Changed

- debuged the hermitAI

- changed StackItem::Ref use a Arc instead of an Rc

- changed StackItem::Table use a Arc instead of an Rc

## [0.2.0] - 2025-02-05

### Changed

- moving to Bevy


## [0.1.0] - 2025-02-01

### Fixed

- `check_for_missing_threads_in_hermit_ai()` was overriding the BTreeSet for the task in `missing` instead of appending the missing tasks to the already detected missing tasks
