# Change Log

Notable changes to the "Ethnopians" crate will be documented in this file.

Check [Keep a Changelog](http://keepachangelog.com/) for recommendations on how to structure this file.

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
