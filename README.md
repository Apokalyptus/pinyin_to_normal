# pinyin_to_normal
This little Project written in Rust impements the conversion of pinyin text (with soundmarks) to normal text (without soundmarks). 
After building the project you can pipe a text document with pinyin trough the pinyin_to_normal tool. The output (stdout) will be normal text. 
I have developed to get a idea of rust development and to convert a list of vocabulary to generate a shadow table in the database for better searching in the application. 
Because Pinyin input is not everywhere possible or tones are not known. 

Usage:

$> cat textfile | pinyin_to_normal

where textfile format should be (one word per line):

lǎopó

lǎogōng

xiānsheng

tàitai

àirén

měinǚ

shuàigē

āyí

...


or 

$> echo lǎopó | pinyin_to_normal
 
