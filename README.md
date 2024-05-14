# What is VNT?

VNT, short for Virtual Network Tool, which is actually a VPN tool to connect devices(PC,phone,cloud node etc) in different networks and create a secure private network.

# Features
- Simple & Easy-to-use
- High Performance
- NAT traversal

# Download
Go to [VNT Release Page](https://github.com/lbl8603/vnt/releases), download a package file according to your operating system.
Then you will get VNT binary program whose file name is `vnt-cli` after extract the package. 

# Getting started (quickly)
Using VNT, it is extremely easy to set up your own network. Just one command and three steps.

### Step 1: make node A to join the private network
```shell
[root@Node-A ~]# ./vnt-cli -k network-id
...
register ip=10.26.0.2 ,netmask=255.255.255.0 ,gateway=10.26.0.1
...
 ====== Connect Successfully ====== 
```

The `-k` option(means _token_) specify the private network identity.

**Note that : devices using the same token will be in one private network.**

The output information shows that Node-A connected to gateway(10.26.0.1) successfully and its virtual ip is 10.26.0.2.
Then We will do the same way on Node-B.

### Step 2: make node B to join the private network
```shell
[root@Node-B ~]# ./vnt-cli -k network-id
...
register ip=10.26.0.3 ,netmask=255.255.255.0 ,gateway=10.26.0.1
...
 ====== Connect Successfully ====== 
```
As you can see, Node-B also joined the private network, whose virtual ip is 10.26.0.3.

### Step 3: Verify
```shell
[root@Node-A ~]# ping 10.26.0.3
PING 10.26.0.3 (10.26.0.3) 56(84) bytes of data.
64 bytes from 10.26.0.3: icmp_seq=1 ttl=64 time=11 ms
64 bytes from 10.26.0.3: icmp_seq=2 ttl=64 time=17 ms
64 bytes from 10.26.0.3: icmp_seq=3 ttl=64 time=21 ms

```
Node-A can communicate with Node-B now. 

So Easy, right?

But what VNT can do more than that. For example, you can set up your own gateway server instead of the shared one just as we use above.



