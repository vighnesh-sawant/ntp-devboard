# A devboard with a gps module and ethernet
This devboard will work as stratum 1 NTP server and has support for PTP also which I plan on supporting. Since this is my first hardware project I have picked simple(?) components  
It will take time from the gps module and serve it as a ptp/ntp server over the lan9250.

# Why
Mostly for fun but I WANT MY LAPTOP TO HAVE MORE ACCURATE TIME THAN MY FRIENDS.
I used to do this same thing with a phone but its accuracy was on the same level as ntp over the public internet  
Thats why I wanted my own STRATUM 1 Server at home, this is also a good project for learning a variety of different stuff ranging from pcb development to writing drivers to messing with ptp.

# How
Get this pcb printed, make sure to get a gps module with a PPS pin and connect the pins via breadboard / soldering   
to the gpio pins of the rp2040, then use my firmware and connect the ethernet cable to your pc.  
Make sure to change the gpio pins in the header to the ones you have connected.
Change your ntp server to point to the rp2040's ip and then done!  
You have your own stratum 1 ntp server  
More instrucions coming after I get my PCB printed!

# Schematic and stuff

<img width="1724" height="1079" alt="image" src="https://github.com/user-attachments/assets/2de5a018-f034-4c0b-a6e2-af85ff9c9852" />
<img width="1326" height="1311" alt="image" src="https://github.com/user-attachments/assets/8eeadad5-4917-4dcd-908f-79f33e1603b3" />
<img width="1307" height="1429" alt="image" src="https://github.com/user-attachments/assets/75401791-9f0b-4857-ae9a-691fbe500bd5" />

# BOM TABLE

