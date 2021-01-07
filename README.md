# rsw-app

To monitor allocations, use pmap or something 

```bash
ps aux | rg 'simulation'
# or
pidof simulation_engine

pmap <pid> | less
top -Hp <pid>
```