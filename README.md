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
<img width="1940" height="1324" alt="image" src="https://github.com/user-attachments/assets/39ccdc05-a4ad-4f0b-900c-25aaf10e868b" />
<img width="1489" height="1215" alt="image" src="https://github.com/user-attachments/assets/d0aa156d-cb5c-4f41-bc15-586fe08fa9c2" />
<img width="1001" height="1048" alt="image" src="https://github.com/user-attachments/assets/7dba7fbb-b57b-4e35-a2e1-f0637feb708b" />

# BOM TABLE
Please look at the bom.xls,lcsv.csv in the root directory!
