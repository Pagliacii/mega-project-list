# Martyr2's Mega Project List

This repository is the Rust implementations of [Martyr2's Mega Project List](https://www.dreamincode.net/forums/topic/78802-martyr2s-mega-project-ideas-list/).

## Table of Contents

+ [Numbers](#Numbers)
  - Find PI to the Nth Digit
  - Fibonacci Sequence
  - Prime Factorization
  - Next Prime Number
  - Find Cost of Tile to Cover W x H Floor
  - Mortgage Calculator
  - Change Return Program
  - Binary to Decimal and Back Converter
  - Calculator
  - Unit Converter (temp, currency, volume, mass and more)
  - Alarm Clock
  - Distance Between Two Cities
  - Credit Card Validator
  - Tax Calculator
  - Factorial Finder
  - Complex Number Algebra
  - Happy Numbers
  - Number Names
+ [Text](#Text)
  - Reverse a String :ballot_box_with_check:
  - Pig Latin :ballot_box_with_check:
  - Count Vowels :ballot_box_with_check:
  - Check if Palindrome :ballot_box_with_check:
  - Count Words in a String :hourglass_flowing_sand:
  - Text Editor
  - RSS Feed Creator
  - Post it Notes Program
  - Quote Tracker (market symbols etc)
  - Guestbook / Journal
  - News Ticker and Game Scores
  - Fortune Teller (Horoscope)
  - Vigenere / Vernam / Ceasar Ciphers
  - Random Gift Suggestions
  - Text to HTML Generator
  - CD Key Generator
  - Regex Query Tool
+ [Networking](#Networking)
  - FTP Program
  - Get Atomic Time from Internet Clock
  - Chat Application (IRC or MSN Style)
  - Fetch Current Weather
  - P2P File Sharing App
  - Port Scanner
  - Mail Checker (POP3 / IMAP)
  - Packet Sniffer
  - Country from IP Lookup
  - Whois Search Tool
  - Zip / Postal Code Lookup
  - Remote Login
  - Site Checker with Time Scheduling
  - Small Web Server
  - Web Bot
+ [Classes](#Classes)
  - Product Inventory Project
  - Movie Store
  - Airline / Hotel Reservation System
  - Student Grade Book Application
  - Bank Account Manager
  - Library Catalog

## Numbers

> This section copies from this [project](https://github.com/miguelgazela/Projects).

+ **Find PI to the Nth Digit** - Enter a number and have the program generate PI up to that many decimal places. Keep a limit to how far the program will go.

+ **Fibonacci Sequence** - Enter a number and have the program generate the Fibonacci sequence to that number or to the Nth number.

+ **Prime Factorization** - Have the user enter a number and find all Prime Factors (if there are any) and display them.

+ **Next Prime Number** - Have the program find prime numbers until the user chooses to stop asking for the next one.

+ **Find Cost of Tile to Cover W x H Floor** - Calculate the total cost of tile it would take to cover a floor plan of width and height, using a cost entered by the user.

+ **Mortgage Calculator** - Calculate the monthly payments of a fixed term mortgage over given Nth terms as a given interest rate. Also figure out how long it will take the user to pay back the loan.

+ **Change Return Program** - The user enters a cost and then the amount of money given. The program will figure out the change and the number of quarters, dimes, nickels, pennies needed for the change.

+ **Binary to Decimal and Back Converter** - Develop a converter to convert a decimal number to binary or a binary number to its decimal equivalent.

+ **Calculator** - A simple calculator to do basic operators. Make it a scientific calculator for added complexity.

+ **Unit Converter (temp, currency, volume, mass and more)** - Converts various units between one another. The user enters the type of unit being entered, the type of unit they want to convert to and then the value. The program will then make the conversion.

+ **Alarm Clock** - A simple clock where it plays a sound after X number of minutes/seconds or at a particular time.

+ **Distance Between Two Cities** - Calculates the distance between two cities and allow the user to specify a unit of distance. This program may require finding coordinates for the cities like latitude and longitude.

+ **Credit Card Validator** - Takes in a credit card number from a common credit card vendor (Visa, MasterCard, American Express, Discoverer) and validates it to make sure that it is a valid number (look into how credit cards use a checksum).

+ **Tax Calculator** - Asks the user to enter a cost and either a country or state tax. It then returns the tax plus the total cost with tax.

+ **Factorial Finder** - The Factorial of a positive integer, n, is defined as the product of the sequence n, n-1, n-2, ...1 and the factorial of zero, 0, is defined as being 1. Solve this using both loops and recursion.

+ **Complex Number Algebra** - Show addition, multiplication, negation, and inversion of complex numbers in separate functions. (Subtraction and division operations can be made with pairs of these operations.) Print the results for each operation tested.

+ **Happy Numbers** - A happy number is defined by the following process. Starting with any positive integer, replace the number by the sum of the squares of its digits, and repeat the process until the number equals 1 (where it will stay), or it loops endlessly in a cycle which does not include 1. Those numbers for which this process ends in 1 are happy numbers, while those that do not end in 1 are unhappy numbers. Take an input number from user, and find first 8 happy numbers from that input.

+ **Number Names** - Show how to spell out a number in English. You can use a preexisting implementation or roll your own, but you should support inputs up to at least one million (or the maximum value of your language's default bounded integer type, if that's less). *Optional: Support for inputs other than positive integers (like zero, negative integers, and floating-point numbers).*

## Text

+ **Reverse a String** - Enter a string and the program will reverse it and print it out. [:page_facing_up:](./reverse-a-string/src/lib.rs)

+ **Pig Latin** - Pig Latin is a game of alterations played on the English language game. To create the Pig Latin form of an English word the initial consonant sound is transposed to the end of the word and an ay is affixed (Ex.: "banana" would yield anana-bay). Read [Wikipedia](https://en.wikipedia.org/wiki/Pig_Latin) for more information on rules. [:page_facing_up:](./pig_latin/src/lib.rs)

+ **Count Vowels** - Enter a string and the program counts the number of vowels in the text. For added complexity have it report a sum of each vowel found. [:page_facing_up:](./vowels_counter/src/lib.rs)

+ **Check if Palindrome** - Checks if the string entered by the user is a palindrome. That is that it reads the same forwards as backwards like "racecar". [:page_facing_up:](./palindrome/src/lib.rs)

+ **Count Words in a String** - Counts the number of individual words in a string. For added complexity read these strings in from a text file and generate a summary.

+ **Text Editor** - Notepad style application that can open, edit, and save text documents. Add syntax highlighting and other features.

+ **RSS Feed Creator** - A program which can read in text from other sources and put it in RSS or Atom news format for syndication.

+ **Post it Notes Program** - A program where you can add text reminders and post them. You can have the program also add popup reminders.

+ **Quote Tracker (market symbols etc)** - A program which can go out and check the current value of stocks for a list of symbols entered by the user. The user can set how often the stocks are checked and the program can show green up and red down arrows to show which direction the stock value has moved.

+ **Guestbook / Journal** - A simple application that allows people to add comments or write journal entries. It can allow comments or not and timestamps for all entries. Could also be made into a shout box.

+ **News Ticker and Game Scores** - A program which sits on your desktop and aggregates news and game scores from various sources on the net. It then scrolls them across the screen on regular intervals.

+ **Fortune Teller (Horoscope)** - A program that checks your horoscope on various astrology sites and puts them together for you each day.

+ **Vigenere / Vernam / Ceasar Ciphers** - Functions for encrypting and decrypting data messages. Then send them to a friend.

+ **Random Gift Suggestions** - Enter various gifts for certain people when you think of them. When its time to give them a gift (xmas, birthday, anniversary) it will randomly pick one and perhaps places you can get it.

+ **Text to HTML Generator** - Converts text files into web HTML files and stylizes them. Great for making online documentation of standard text documentation.

+ **CD Key Generator** - Generates a unique key for your applications to use based on some arbitrary algorithm that you can specify. Great for software developers looking to make shareware that can be activated.

+ **Regex Query Tool** - A tool that allows the user to enter a text string and then in a separate control enter a regex pattern. It will run the regular expression against the source text and return any matches or flag errors in the regular expression.

## Networking

+ **FTP Program** - A file transfer program which can transfer files back and forth from a remote web server.

+ **Get Atomic Time from Internet Clock** - This program will get the true atomic time from an atomic time clock on the Internet. There are various clocks across the world. Do a search for a list of them.

+ **Chat Application (IRC or MSN Style)** - Create a chat application that can create simple chat rooms like on Internet Relay Chat (IRC) or a more direct chatting style like MSN. For added complexity, create your own protocol to facilitate this chatting.

+ **Fetch Current Weather** - Get the current weather for a given zip/postal code.

+ **P2P File Sharing App** - Create a program like LimeWire, FrostWire, Bearshare, or a torrent style application.

+ **Port Scanner** - Enter an IP address and a port range where the program will then attempt to find open ports on the given computer by connecting to each of them. On any successful connections mark the port as open.

+ **Mail Checker (POP3 / IMAP)** - The user enters various account information include web server and IP, protocol type (POP3 or IMAP) and the application will check for email on several accounts at a given interval.

+ **Packet Sniffer** - A utility program that will read packets coming in and out of the machine along with related information like destination and payload size.

+ **Country from IP Lookup** - Enter an IP address and find the country that IP is registered in.

+ **Whois Search Tool** - Enter an IP or host address and have it look it up through whois and return the results to you.

+ **Zip / Postal Code Lookup** - Enter a zip or postal code and have it return which city/cities that are in that zip code.

+ **Remote Login** - Create a remote desktop style application which can see and control the remote computer (given you have permissions). It may require the use of your own private network and a second computer to test with.

+ **Site Checker with Time Scheduling** - An application that attempts to connect to a website or server every so many minutes or a given time and check if it is up. If it is down, it will notify you by email or by posting a notice on screen.

+ **Small Web Server** - A simple web server that can serve HTML files that contain Javascript and other forms of non-code executing code. Added complexity would be to try and implement streaming video, create a server-side language, or serve up other stream types.

+ **Web Bot** - An automated program which carries out tasks on the web including checking websites, page scraping, and summarization of data or web posting.

## Classes

+ **Product Inventory Project** - Create an application which manages an inventory of products. Create a product class which has a price, id, and quantity on hand. Then create an inventory class which keeps track of various products and can sum up the inventory value.

+ **Movie Store** - Manage video rentals and controls when videos are checked out, due to return, overdue fees and for added complexity create a summary of those accounts which are overdue for contact.

+ **Airline / Hotel Reservation System** - Create a reservation system which books airline seats or hotel rooms. It charges various rates for particular sections of the plane or hotel. Example, first class is going to cost more than coach. Hotel rooms have penthouse suites which cost more. Keep track of when rooms will be available and can be scheduled.

+ **Student Grade Book Application** - Keep track of students (with a student class that has their name, average, and scores) in a class and their grades. Assign their scores on tests and assignments to the students and figure out their average and grade for the class. For added complexity put the students on a bell curve.

+ **Bank Account Manager** - Create a class called "Account" which will be an abstract class for three other classes called "CheckingAccount", "SavingAccount" and "BusinessAccount". Manage credits and debits from these accounts through an ATM style program.

+ **Library Catalog** - Create a book class with a title, page count, ISBN and whether or not it is checked out or not. Manage a collection of various books and allow the user to check out books or return books. For added complexity generate a report of those books overdue and any fees. Also allow users to put books on reserve.

## License

:memo:[MIT](./LICENSE)

