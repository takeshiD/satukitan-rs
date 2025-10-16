# Satu kitan script Language design
## DataType

```sat
# number
rv  # 0
ru  # 1
ra  # 2
ro  # 3
re  # 4
ri  # 5
rya # 6
ryu # 7
ryo # 8
rye # 9
#ta # 10

# boolean
ga      # false
me      # true

# String
""              # empty string
"sanasapotan"   # string
```

## ContainerType
### List
```sat
# list
[ra ra ra ru re]  # [2 2 2 1 4]

# sort list
fanitas [ra ra ra ru re]    # [2 2 2 1 4]
>> [ru ra ra ra re]         # [1 2 2 2 4]

# length of list
rakas [ra ra ra ru re]
>> ri                       # 5
```

## Binary Operation
```sat
# add
ritas ra ru     # add 1 2
>> ro           # 3

# sub
matyes ra ru    # sub 2 1
>> ru           # 1

# multiple
nitas ra ra     # mul 2 2
>> re           # 4

# logic and
teses ga me    # and false true
>> ga

# logic or
kenus ga me    # or false true
>> me

# comparison
ditas ra ru    # ra < ru
fityes ra ru   # ra > ra
gatas ra ru    # ra == ru

ditasgata ra ru   # ra <= ra
fityesgata ra ru  # ra >= ra
```

## Condition
```sat
nobu ga 
    (sipus "sana sapotav!") # then sentence
    (sipus "hello world")   # else sentence
```

## Define variable, function
```sat
# Bound variable
gakas x ra      # x = 2
gakas y ra      # x = 2

# Bound closure
gakas f ((x1 x2) (nitas (matyes x1 x1) x2))   # f = (x1, x2) => (x1 - x1) * x2  # typescript arrow function like

# Function Application
f ra ru

# Recursive Function
gakasdenu fibo (n) (
    nobu ditas n ra     # n < 2
    (ru)                # 1
    (ritas fibo matyes n 1 fibo matyes n 2)
)
```

## Standard Output
`sipus` is standard output funcntion.

```sat
sipus "sana sapotav!"
>> sana sapotav!

sipus ritas ra ru
>> ro
```
