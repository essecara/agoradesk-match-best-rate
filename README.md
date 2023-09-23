
# Agoradesk-match-best-price

Do you also dislike if some trader sets their rate to 0.01 cents bellow yours? Even more if you don't pose any competition to him? If yes, this app is for you. This app let's you match the best offer on currency / method obeying limits you set.

Running this for several hours resulted in strange anomalies .. several users had very similar prices and we didn't go lower ... invisible hand of the market found the best rate :-) 

see: https://imgur.com/a/6wbYiQi


## Configuration

**notes:**

 1. *limit* if > 1, fixed priced is used as the lowest amount possible that is obeyed
 2. *margin* is basically a percentage in that other format ... 5% = 1.05 .... if you use this, you may only use XMR/EUR rate from kraken .. 
 3. *skip_ads*  - if you wanna exclude some ads from price match ( eg. your other ads with even lower rate )

```
    {
    	"ad": "your advertisement id",
    	"apikey": "your api key",
    	"currency" : "USD",
    	"method": "zelle",
    	"margin" 1.1,
    	"limit": -1,
    	"skip_ads": [
    		"ad1_id",
    		"ad2_id"
    	]
    }
```

## how to compile & run
rustc & cargo recommended

```cargo run -- /path/to/config.json```

Compiled:
`` ./app /path/to/config.json ````
