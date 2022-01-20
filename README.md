# testspecgen

## Installation

### Build requirements

- LLVM and Clang

```sh
cargo install --git https://github.com/koyashiro/testspecgen
```

## Usage

```
USAGE:
    testspecgen [OPTIONS] <INPUT> <OUTPUT>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --body-bg-color <BODY_BG_COLOR>                           [env: BODY_BG_COLOR=]  [default: 0xffffff]
        --body-font-color <BODY_FONT_COLOR>                       [env: BODY_FONT_COLOR=]  [default: 0x000000]
        --border-color <BORDER_COLOR>                             [env: BORDER_COLOR=]  [default: 0x5b9bd5]
        --confirmations-header <CONFIRMATIONS_HEADER>             [env: CONFIRMATIONS_HEADER=]  [default: Confirmations]
        --confirmations-item-width <CONFIRMATIONS_ITEM_WIDTH>     [env: CONFIRMATIONS_ITEM_WIDTH=]  [default: 60]
        --font-family <FONT_FAMILY>                               [env: FONT_FAMILY=]  [default: Yu Gothic]
    -f, --format <FORMAT>                                         [env: FORMAT=]  [default: markdown]
        --header-bg-color <HEADER_BG_COLOR>                       [env: HEADER_BG_COLOR=]  [default: 0x5b9bd5]
        --header-font-color <HEADER_FONT_COLOR>                   [env: HEADER_FONT_COLOR=]  [default: 0xffffff]
        --no-header <NO_HEADER>                                   [env: NO_HEADER=]  [default: No.]
        --no-width <NO_WIDTH>                                     [env: NO_WIDTH=]  [default: 8]
        --operations-header <OPERATIONS_HEADER>                   [env: OPERATIONS_HEADER=]  [default: Operations]
        --operations-item-width <OPERATIONS_ITEM_WIDTH>           [env: OPERATIONS_ITEM_WIDTH=]  [default: 60]
        --operator-header <OPERATOR_HEADER>                       [env: OPERATOR_HEADER=]  [default: Operator]
        --operator-width <OPERATOR_WIDTH>                         [env: OPERATOR_WIDTH=]  [default: 12]
        --primary-item-header <PRIMARY_ITEM_HEADER>               [env: PRIMARY_ITEM_HEADER=]  [default: Primary Item]
        --primary-item-width <PRIMARY_ITEM_WIDTH>                 [env: PRIMARY_ITEM_WIDTH=]  [default: 16]
        --remarks-header <REMARKS_HEADER>                         [env: REMARKS_HEADER=]  [default: Remarks]
        --remarks-item-width <REMARKS_ITEM_WIDTH>                 [env: REMARKS_ITEM_WIDTH=]  [default: 60]
        --result-header <RESULT_HEADER>                           [env: RESULT_HEADER=]  [default: Result]
        --result-width <RESULT_WIDTH>                             [env: RESULT_WIDTH=]  [default: 8]
        --secondary-item-header <SECONDARY_ITEM_HEADER>
             [env: SECONDARY_ITEM_HEADER=]  [default: Secondary Item]

        --secondary-item-width <SECONDARY_ITEM_WIDTH>             [env: SECONDARY_ITEM_WIDTH=]  [default: 16]
        --tertiary-item-header <TERTIARY_ITEM_HEADER>             [env: TERTIARY_ITEM_HEADER=]  [default: Tertiary Item]
        --tertiary-item-width <TERTIARY_ITEM_WIDTH>               [env: TERTIARY_ITEM_WIDTH=]  [default: 16]

ARGS:
    <INPUT>
    <OUTPUT>
```

## Example

<details>
<summary>example.yml</summary>

```yml
# yaml-language-server: $schema=https://raw.githubusercontent.com/koyashiro/testspecgen/main/docs/schema.json

title: Spec title
cases:
  - title: Primary 1
    children:
      - title: Secondary 1-1
        children:
          - title: Tertiary 1-1-1
            operations:
              - Operation 1-1-1-1
              - Operation 1-1-1-2
              - Operation 1-1-1-3
            confirmations:
              - Confirmation 1-1-1-1
              - Confirmation 1-1-1-2
              - Confirmation 1-1-1-3
            remarks:
              - Remark 1-1-1-1
              - Remark 1-1-1-2
              - Remark 1-1-1-3
          - title: Tertiary 1-1-2
            operations:
              - Operation 1-1-2-1
              - Operation 1-1-2-2
              - Operation 1-1-2-3
            confirmations:
              - Confirmation 1-1-2-1
              - Confirmation 1-1-2-2
              - Confirmation 1-1-2-3
            remarks:
              - Remark 1-1-2-1
              - Remark 1-1-2-2
              - Remark 1-1-2-3
      - title: Secondary 1-2
        children:
          - title: Tertiary 1-2-1
            operations:
              - Operation 1-2-1-1
              - Operation 1-2-1-2
              - Operation 1-2-1-3
            confirmations:
              - Confirmation 1-2-1-1
              - Confirmation 1-2-1-2
              - Confirmation 1-2-1-3
            remarks:
              - Remark 1-2-1-1
              - Remark 1-2-1-2
              - Remark 1-2-1-3
          - title: Tertiary 1-2-2
            operations:
              - Operation 1-2-2-1
              - Operation 1-2-2-2
              - Operation 1-2-2-3
            confirmations:
              - Confirmation 1-2-2-1
              - Confirmation 1-2-2-2
              - Confirmation 1-2-2-3
            remarks:
              - Remark 1-2-2-1
              - Remark 1-2-2-2
              - Remark 1-2-2-3
  - title: Primary 2
    children:
      - title: Secondary 2-1
        children:
          - title: Tertiary 2-1-1
            operations:
              - Operation 2-1-1-1
              - Operation 2-1-1-2
              - Operation 2-1-1-3
            confirmations:
              - Confirmation 2-1-1-1
              - Confirmation 2-1-1-2
              - Confirmation 2-1-1-3
            remarks:
              - Remark 2-1-1-1
              - Remark 2-1-1-2
              - Remark 2-1-1-3
          - title: Tertiary 2-1-2
            operations:
              - Operation 2-1-2-1
              - Operation 2-1-2-2
              - Operation 2-1-2-3
            confirmations:
              - Confirmation 2-1-2-1
              - Confirmation 2-1-2-2
              - Confirmation 2-1-2-3
            remarks:
              - Remark 2-1-2-1
              - Remark 2-1-2-2
              - Remark 2-1-2-3
      - title: Secondary 2-2
        children:
          - title: Tertiary 2-2-1
            operations:
              - Operation 2-2-1-1
              - Operation 2-2-1-2
              - Operation 2-2-1-3
            confirmations:
              - Confirmation 2-2-1-1
              - Confirmation 2-2-1-2
              - Confirmation 2-2-1-3
            remarks:
              - Remark 2-2-1-1
              - Remark 2-2-1-2
              - Remark 2-2-1-3
          - title: Tertiary 2-2-2
            operations:
              - Operation 2-2-2-1
              - Operation 2-2-2-2
              - Operation 2-2-2-3
            confirmations:
              - Confirmation 2-2-2-1
              - Confirmation 2-2-2-2
              - Confirmation 2-2-2-3
            remarks:
              - Remark 2-2-2-1
              - Remark 2-2-2-2
              - Remark 2-2-2-3
```

</details>

### Generate Markdown

```sh
testspecgen -f markdown example.yml example.md
```

<details>
<summary>example.md</summary>

```md
# Spec title

## Primary 1

### Secondary 1-1

#### Tertiary 1-1-1

##### Operations

1. Operation 1-1-1-1
2. Operation 1-1-1-2
3. Operation 1-1-1-3

##### Confirmation

- [ ] Confirmation 1-1-1-1
- [ ] Confirmation 1-1-1-2
- [ ] Confirmation 1-1-1-3

##### Remarks

- Remark 1-1-1-1
- Remark 1-1-1-2
- Remark 1-1-1-3

#### Tertiary 1-1-2

##### Operations

1. Operation 1-1-2-1
2. Operation 1-1-2-2
3. Operation 1-1-2-3

##### Confirmation

- [ ] Confirmation 1-1-2-1
- [ ] Confirmation 1-1-2-2
- [ ] Confirmation 1-1-2-3

##### Remarks

- Remark 1-1-2-1
- Remark 1-1-2-2
- Remark 1-1-2-3

### Secondary 1-2

#### Tertiary 1-2-1

##### Operations

1. Operation 1-2-1-1
2. Operation 1-2-1-2
3. Operation 1-2-1-3

##### Confirmation

- [ ] Confirmation 1-2-1-1
- [ ] Confirmation 1-2-1-2
- [ ] Confirmation 1-2-1-3

##### Remarks

- Remark 1-2-1-1
- Remark 1-2-1-2
- Remark 1-2-1-3

#### Tertiary 1-2-2

##### Operations

1. Operation 1-2-2-1
2. Operation 1-2-2-2
3. Operation 1-2-2-3

##### Confirmation

- [ ] Confirmation 1-2-2-1
- [ ] Confirmation 1-2-2-2
- [ ] Confirmation 1-2-2-3

##### Remarks

- Remark 1-2-2-1
- Remark 1-2-2-2
- Remark 1-2-2-3

## Primary 2

### Secondary 2-1

#### Tertiary 2-1-1

##### Operations

1. Operation 2-1-1-1
2. Operation 2-1-1-2
3. Operation 2-1-1-3

##### Confirmation

- [ ] Confirmation 2-1-1-1
- [ ] Confirmation 2-1-1-2
- [ ] Confirmation 2-1-1-3

##### Remarks

- Remark 2-1-1-1
- Remark 2-1-1-2
- Remark 2-1-1-3

#### Tertiary 2-1-2

##### Operations

1. Operation 2-1-2-1
2. Operation 2-1-2-2
3. Operation 2-1-2-3

##### Confirmation

- [ ] Confirmation 2-1-2-1
- [ ] Confirmation 2-1-2-2
- [ ] Confirmation 2-1-2-3

##### Remarks

- Remark 2-1-2-1
- Remark 2-1-2-2
- Remark 2-1-2-3

### Secondary 2-2

#### Tertiary 2-2-1

##### Operations

1. Operation 2-2-1-1
2. Operation 2-2-1-2
3. Operation 2-2-1-3

##### Confirmation

- [ ] Confirmation 2-2-1-1
- [ ] Confirmation 2-2-1-2
- [ ] Confirmation 2-2-1-3

##### Remarks

- Remark 2-2-1-1
- Remark 2-2-1-2
- Remark 2-2-1-3

#### Tertiary 2-2-2

##### Operations

1. Operation 2-2-2-1
2. Operation 2-2-2-2
3. Operation 2-2-2-3

##### Confirmation

- [ ] Confirmation 2-2-2-1
- [ ] Confirmation 2-2-2-2
- [ ] Confirmation 2-2-2-3

##### Remarks

- Remark 2-2-2-1
- Remark 2-2-2-2
- Remark 2-2-2-3
```

</details>

### Generate Excel

```sh
testspecgen -f excel example.yml example.xlsx
```

![image](https://user-images.githubusercontent.com/6698252/150153778-96292621-2244-4fe8-97c2-3e62ef98b38b.png)
