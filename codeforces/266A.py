z=input
z()
a=z()
last = 'n'
count = 0
for value in a:
    if(last != value):
        count+=1
        last = value

print(len(a) - count)

# r=input;r();c=r();print(sum(p==n for p,n in zip(c[1:],c)))
