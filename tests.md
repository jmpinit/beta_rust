Hardware
--------

After every instruction:
	* assert!(PC % 4 == 0)
	* assert!(SP % 4 == 0) 

Software
--------

### Procedures
* args passed on stack
* return address passed in LP
* result in R0
* registers other than R0 unchanged
