var searchIndex = {};
searchIndex['redux'] = {"items":[[0,"","redux","Adaptive arithmetic compression library.",null,null],[4,"Error","","Possible errors that occur throughout this library",null,null],[13,"Eof","","The input stream has ended (unexpectedly)",0,null],[13,"InvalidInput","","An invalid combination of data has occured on the input that the library was unable to process.",0,null],[13,"IoError","","An I/O error occured.",0,null],[5,"compress","","Compresses `istream` into `ostream` using the given `model`.\nReturns the number of bytes both in the decompressed and compressed stream.",null,{"inputs":[{"name":"read"},{"name":"write"},{"name":"box"}],"output":{"name":"result"}}],[5,"decompress","","Decompresses `istream` into `ostream` using the given `model`.\nReturns the number of bytes both in the compressed and decompressed stream.",null,{"inputs":[{"name":"read"},{"name":"write"},{"name":"box"}],"output":{"name":"result"}}],[0,"bitio","","Bit level I/O operations.",null,null],[3,"BitWriter","redux::bitio","A class for wrapping a byte output stream in a bit based interface.",null,null],[3,"BitReader","","A class for wrapping a byte input stream in a bit based interface.",null,null],[8,"ByteCount","","A trait for counting the number of bytes flowing trough a `Read` or `Write` implementation.",null,null],[10,"get_count","","Returns the number of bytes in this stream.",1,{"inputs":[{"name":"bytecount"}],"output":{"name":"u64"}}],[8,"ByteWrite","","A trait for object that allow writing one byte at a time.",null,null],[10,"write_byte","","Writes a single byte to the output.",2,{"inputs":[{"name":"bytewrite"},{"name":"u8"}],"output":{"name":"result"}}],[8,"BitWrite","","A trait for object that allow writing one bit at a time.",null,null],[10,"write_bit","","Writes a single bit to the output.",3,{"inputs":[{"name":"bitwrite"},{"name":"bool"}],"output":{"name":"result"}}],[10,"flush_bits","","Flushes all remaining bits to the output after the last whole octet.",3,{"inputs":[{"name":"bitwrite"}],"output":{"name":"result"}}],[8,"ByteRead","","A trait for object that allow reading one byte at a time.",null,null],[10,"read_byte","","Reads a single byte from the input.",4,{"inputs":[{"name":"byteread"}],"output":{"name":"result"}}],[8,"BitRead","","A trait for object that allow reading one bit at a time.",null,null],[10,"read_bit","","Reads a single bit from the input.",5,{"inputs":[{"name":"bitread"}],"output":{"name":"result"}}],[11,"new","","Creates a new instance by wrapping a byte output stream.",6,{"inputs":[{"name":"bitwriter"},{"name":"write"}],"output":{"name":"bitwriter"}}],[11,"get_count","","Returns the number of bytes written to the output.",6,{"inputs":[{"name":"bitwriter"}],"output":{"name":"u64"}}],[11,"write_byte","","",6,{"inputs":[{"name":"bitwriter"},{"name":"u8"}],"output":{"name":"result"}}],[11,"write_bit","","",6,{"inputs":[{"name":"bitwriter"},{"name":"bool"}],"output":{"name":"result"}}],[11,"flush_bits","","",6,{"inputs":[{"name":"bitwriter"}],"output":{"name":"result"}}],[11,"new","","Creates a new instance by wrapping a byte input stream.",7,{"inputs":[{"name":"bitreader"},{"name":"read"}],"output":{"name":"bitreader"}}],[11,"get_count","","Returns the number of bytes written to the output.",7,{"inputs":[{"name":"bitreader"}],"output":{"name":"u64"}}],[11,"read_byte","","",7,{"inputs":[{"name":"bitreader"}],"output":{"name":"result"}}],[11,"read_bit","","",7,{"inputs":[{"name":"bitreader"}],"output":{"name":"result"}}],[0,"codec","redux","Model-independent compression and decompression module.",null,null],[3,"Codec","redux::codec","The current state of the encoder and decoder.",null,null],[11,"new","","Creates and initializes the codec for encoding or decoding.",8,{"inputs":[{"name":"codec"},{"name":"box"}],"output":{"name":"codec"}}],[11,"compress_symbol","","Compresses a symbol and outputs some bits depending on the state of the codec.",8,{"inputs":[{"name":"codec"},{"name":"usize"},{"name":"bitwrite"}],"output":{"name":"result"}}],[11,"compress_bytes","","Compresses an entire byte stream outputting the EOF symbol and all bits for unambigous encoding.",8,{"inputs":[{"name":"codec"},{"name":"byteread"},{"name":"bitwrite"}],"output":{"name":"result"}}],[11,"decompress_symbol","","Decompresses a symbol reading some bits until the symbol can be decoded.",8,{"inputs":[{"name":"codec"},{"name":"bitread"}],"output":{"name":"result"}}],[11,"decompress_bytes","","Decompresses a whole bit stream until the EOF symbol is found.",8,{"inputs":[{"name":"codec"},{"name":"bitread"},{"name":"bytewrite"}],"output":{"name":"result"}}],[0,"model","redux","Symbol frequency distribution models.",null,null],[3,"AdaptiveLinearModel","redux::model","Adaptive model that uses a simple array for cumulative freq\nand simple, but slow linear algorithms for operations.",null,null],[3,"AdaptiveTreeModel","","Adaptive model that uses a Binary Indexed Tree for storing cumulative frequencies.",null,null],[3,"Parameters","","Model parameters that specifies the common property of the models.",null,null],[12,"symbol_bits","","Bit width of the symbols being encoded.\nUsually 8 for byte oriented inputs.",9,null],[12,"symbol_eof","","Code for the EOF symbol.\nThis is the next symbol code after the valid symbols to encode.",9,null],[12,"symbol_count","","Number of possible symbols including the EOF symbol.",9,null],[12,"freq_bits","","Number of bits representing symbol frequencies.",9,null],[12,"freq_max","","Maximum cumulated frequency value for symbols.",9,null],[12,"code_bits","","Number of bits representing the current code ranges.",9,null],[12,"code_min","","Minimum value for code range.\nThis is always zero.",9,null],[12,"code_one_fourth","","Delimiter for the one fourth of the valid code range.",9,null],[12,"code_half","","Delimiter for the half of the valid code range.",9,null],[12,"code_three_fourths","","Delimiter for the three fourths of the valid code range.",9,null],[12,"code_max","","Upper limit of the valid code range.",9,null],[11,"new","","Initializes the model with the given parameters.",10,{"inputs":[{"name":"adaptivelinearmodel"},{"name":"parameters"}],"output":{"name":"box"}}],[11,"parameters","","",10,{"inputs":[{"name":"adaptivelinearmodel"}],"output":{"name":"parameters"}}],[11,"total_frequency","","",10,{"inputs":[{"name":"adaptivelinearmodel"}],"output":{"name":"u64"}}],[11,"get_frequency","","",10,{"inputs":[{"name":"adaptivelinearmodel"},{"name":"usize"}],"output":{"name":"result"}}],[11,"get_symbol","","",10,{"inputs":[{"name":"adaptivelinearmodel"},{"name":"u64"}],"output":{"name":"result"}}],[11,"get_freq_table","","",10,{"inputs":[{"name":"adaptivelinearmodel"}],"output":{"name":"vec"}}],[11,"new","","Initializes the model with the given parameters.",11,{"inputs":[{"name":"adaptivetreemodel"},{"name":"parameters"}],"output":{"name":"box"}}],[11,"parameters","","",11,{"inputs":[{"name":"adaptivetreemodel"}],"output":{"name":"parameters"}}],[11,"total_frequency","","",11,{"inputs":[{"name":"adaptivetreemodel"}],"output":{"name":"u64"}}],[11,"get_frequency","","",11,{"inputs":[{"name":"adaptivetreemodel"},{"name":"usize"}],"output":{"name":"result"}}],[11,"get_symbol","","",11,{"inputs":[{"name":"adaptivetreemodel"},{"name":"u64"}],"output":{"name":"result"}}],[11,"get_freq_table","","",11,{"inputs":[{"name":"adaptivetreemodel"}],"output":{"name":"vec"}}],[8,"Model","","Trait for the probability models behind arithmetic coding.\nPossible implementations may include static models with fixed probabilities\nor and adaptive model that continuously updates cumulative frequencies.",null,null],[10,"parameters","","Returns the arithmetic compression parameters.",12,{"inputs":[{"name":"model"}],"output":{"name":"parameters"}}],[10,"total_frequency","","Returns the maximum cumulative frequency.",12,{"inputs":[{"name":"model"}],"output":{"name":"u64"}}],[10,"get_frequency","","Returns the cumulative frequency range for the given input symbol.",12,{"inputs":[{"name":"model"},{"name":"usize"}],"output":{"name":"result"}}],[10,"get_symbol","","Returns the symbol that corresponds to the given cumulative frequency.",12,{"inputs":[{"name":"model"},{"name":"u64"}],"output":{"name":"result"}}],[10,"get_freq_table","","Returns the cumulative frequency table for debugging purposes.",12,{"inputs":[{"name":"model"}],"output":{"name":"vec"}}],[11,"clone","","",9,{"inputs":[{"name":"parameters"}],"output":{"name":"parameters"}}],[11,"new","","Calculates all parameter values based on the `symbol`, `frequency` and `code` width.",9,{"inputs":[{"name":"parameters"},{"name":"usize"},{"name":"usize"},{"name":"usize"}],"output":{"name":"result"}}],[6,"Result","redux","Specialized `Result` type for the `redux` library.",null,null],[11,"fmt","","",0,{"inputs":[{"name":"error"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"fmt","","",0,{"inputs":[{"name":"error"},{"name":"formatter"}],"output":{"name":"result"}}]],"paths":[[4,"Error"],[8,"ByteCount"],[8,"ByteWrite"],[8,"BitWrite"],[8,"ByteRead"],[8,"BitRead"],[3,"BitWriter"],[3,"BitReader"],[3,"Codec"],[3,"Parameters"],[3,"AdaptiveLinearModel"],[3,"AdaptiveTreeModel"],[8,"Model"]]};
initSearch(searchIndex);
