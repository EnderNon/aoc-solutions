import json
file = open("list.txt","r")
l1 = []
l2 = []
for i in file:
    a,b = i.strip().split("   ")
    a = int(a)
    b = int(b)
    l1.append(a)
    l2.append(b)

file.close()

l1.sort()
l2.sort()

with open('l1.json', 'w') as l1file:
    json.dump(l1,l1file)
l1file.close()

with open('l2.json', 'w') as l2file:
    json.dump(l2,l2file)
l2file.close()