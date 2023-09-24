#Misanthropy
Use https://github.com/Aviuz/BadPeople/wiki/Morality-progression as a base

* have adjustments to points where they tiping points, keep the 0% as the point it regressees to, hanging it just add needles complexity.
* multiply `Misanthropy` by 0.83 per day to get it from 100% to below 0.5% in 30 days

# faminurarity
##Meetples
The people a character meets is a meetple.

story evey meetple they enounter for the day. then at the end of the day put them in a Bloom Filter.
keep the Daily Bloom filters for a month
if the meetple is in most(setable paramirter) of the last month's days then they are starting to look familour. 
**store them in the monthly BloomFilter(needs to use a diff hash other words the monthly bloom wouldn't be able to diffrangate between.) 
***alternatively you don't need to test when meeting a meetple if they are common in the daily Bloom filters. just go though the dailies and if a bit is set in enough of them then set it in the
t in the monthly bloom)

# rooms
for objects that define a rooms claims all the area in Line of Sight to the object, including the things that finally block it.
*if anouther object that defines the same room type in in the claimed area then their claimed areas are combined to define the room.
**resonate objects.  Don't define a room by themselfs but if they a claimed by the approate room type then they claim an area for that room like an room defineing object.

#conversations
mark places where the speaker can be interuped in their dialogue.

pull the group on who want to take turn then use a lottary to select the next speaker.
*if no one want to talk then have a silince/awkward silence
**how do you desice what type of silince
***Have is marked in the dialogue?
***Have an dialogue maganger agent decide?
***combinatio of the two where the dialogue give weights for the options. e,g, +3 to akward silince.
## signaling
[In contrast, when the current speaker and the listeners open their mouths narrowly after opening them narrowly and then widely, the utterance interval tends to be long.](Prediction of Who Will Be Next Speaker and When Using Mouth-Opening Pattern in Multi-Party Conversation)
###Speaks signals for for turn taking
[A key finding of the MOTP analysis is that the current speaker often keeps her mouth narrow-open during turn-keeping and starts to close it after opening it narrowly or continues to open it widely during turn-changing.]
(Prediction of Who Will Be Next Speaker and When Using Mouth-Opening Pattern in Multi-Party Conversation)
### listers signal to take the turn
[The next speaker often starts to open her mouth narrowly after closing it during turn-changing. Moreover, when the current speaker starts to close her mouth after opening it narrowly in turn-keeping, the utterance interval tends to be short.](Prediction of Who Will Be Next Speaker and When Using Mouth-Opening Pattern in Multi-Party Conversation)
##conversational turntaking
Lottory with the amount od tickets being based on the social status of the participents

# trade value of items
the base value is the time it could take to make it.
the base value is adjust base of how much he has stored.
* option A multiply base_value by the % of storage not used by the same type of item.
* option B multiply base_value by (1 - (storage_used_by_item_type / (total_stotage - storage_used_by_other_item_types)))
* option C ((option_A + option_B) / 2)
