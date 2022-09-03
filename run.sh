#!/bin/bash  
 
start=0
range=10000

for((i=1;i<=100;i++));  
do   
start=$(expr $start + $range);
echo $start;  
dfx canister call crud execute '('$start','$range')';
done  