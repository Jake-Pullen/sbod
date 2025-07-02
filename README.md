# Spell Brigade Offensive Defence

## a little rust learning

The initial concept and MVP of this small rust CLI application is to be a little tool to help with an in game calculation.  
The Relic [Offensive Defence](https://thespellbrigade.wiki.gg/wiki/Relics) gives you extra spell power but at the cost of movement speed...  
The way this is calculated though, you can go into negative movement speeds.  
With enough negative speed you can go backwards, and start increasing your speed (in reverse).
Negating any negative effects of the relic.

So MVP success criteriea is:  
- input your current speed and your goal speed (or default to -130) and the output is how much armour you need to get there. 

