import sys
import re
from termcolor import colored

def error(lineno, msg):
	raise Exception("error on line "+lineno+":", msg)

if len(sys.argv) == 2:  # check for filename argument
    filename = sys.argv[1]
    with open(filename) as f:
        source = f.readlines()

	for line_number, line in enumerate(source):
		# get rid of comments
		line = line.split("|")[0].strip()

		# directive?
		m = re.search('(?<=\.)\w+', line)
		if(m):
			print colored(line, "yellow")
			directive = m.group(0)
			if directive == "include":
				print "INCLUDE"
			elif directive == "align":
				value = 4

				m = re.search(
					r"\.align\s?" +
					r"(?P<expression>.*)\s?"	# expression
				, line)

				if(m):
					expression = m.groupdict()['expression']

					try:
						value = eval(expression)
					except:
						error("Malformed expression")

				print "ALIGN", (value)
			elif directive == "ascii":
				print "ASCII"
			elif directive == "text":
				print "TEXT"
			elif directive == "macro":
				m = re.search(
					r"\.macro\s?" + 
					r"(?P<name>\w+)" +					# name
					r"\s?\((?P<args>[\w, ]*)\)\s" + 	# arguments
					r"\s?(?P<value>.*)\s?"				# value
				, line)

				args = []
				if(m):
					parsed = m.groupdict()
					name = parsed['name']
					args += parsed['args'].split(",")
					args = [arg.strip() for arg in args]
					value = parsed['value']

					#print "MACRO", (name, args, value)
				else:
					error("Malformed macro")
			else:
				error("Unrecognized directive" + directive)

else:
    print "usage: ./uasm.py <source file>"

