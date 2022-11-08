# Recovery from bricked Blockchain

In the unfortunate even that the blockchain stops the block production, the blockchain becomes bricked after just 1 epoch time (15 minutes).  
The situation can happen because a broken runtime update or because the main part of validators goes off-line for whatever reason.   
Deleting the blockchain is not applicable in a production environment and there are only 2 different solutions:
- Hard Spoon - a new blockchain pulling back the previous balances.
- Time regression of the majority of the validators (and fast advancement to the real time).

# Time Regression

- The majority of validators should change the system time to the same of the last written block within a few second tolerance from each other. The block productions should restart in a few seconds.  
- The time cannot be kept back for ever, so an utility called "warptime.py" will move forward the time 3 times faster of the usual one.  

# Running Time Warp
Execute the following command on all the validator at the same time. crontab/at utility in Linux can be used for the purpose.
```bash
python3 warptime.py
```
Once it's stopped, execute it again till the starting point is almost the same.

Finally syncronize the system time with a NTP server again. The blockchain will be up and running again properly with the correct system time.



