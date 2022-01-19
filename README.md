# testspecgen

## Installation

### Build requirements

- LLVM and Clang

```sh
cargo install --git https://github.com/koyashiro/testspecgen
```

## Usage

```
testspecgen [OPTIONS] <INPUT> <OUTPUT>
```

```
OPTIONS:
        --confirmations-column <CONFIRMATIONS_COLUMN>       [env: CONFIRMATIONS_COLUMN=]  [default: Confirmations]
        --font <FONT>                                       [env: FONT=]  [default: Yu Gothic]
    -f, --format <FORMAT>                                   [env: FORMAT=]  [default: markdown]
        --no-column <NO_COLUMN>                             [env: NO_COLUMN=]  [default: No.]
        --operations-column <OPERATIONS_COLUMN>             [env: OPERATIONS_COLUMN=]  [default: Operations]
        --operator-column <OPERATOR_COLUMN>                 [env: OPERATOR_COLUMN=]  [default: Operator]
        --primary-item-column <PRIMARY_ITEM_COLUMN>         [env: PRIMARY_ITEM_COLUMN=]  [default: Primary Item]
        --remarks-column <REMARKS_COLUMN>                   [env: REMARKS_COLUMN=]  [default: Remarks]
        --result-column <RESULT_COLUMN>                     [env: RESULT_COLUMN=]  [default: Result]
        --secondary-item-column <SECONDARY_ITEM_COLUMN>     [env: SECONDARY_ITEM_COLUMN=]  [default: Secondary Item]
        --tertiary-item-column <TERTIARY_ITEM_COLUMN>       [env: TERTIARY_ITEM_COLUMN=]  [default: Tertiary Item]
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
