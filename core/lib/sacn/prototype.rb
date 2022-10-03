def packet(seq_number)
  data = []

  ## Root Layer
  # pre amble size
  data << 0x0010
  # post amble size
  data << 0x0000

  # packet identifier
  data << 0x4153
  data << 0x432d
  data << 0x4531
  data << 0x2e31
  data << 0x3700
  data << 0x0000

  # flags and length
  data << 0x726E

  # vector E131_ROOT_DATA
  data << 0x0000
  data << 0x0004

  # CID - randomly generated for this example
  data << 0xe68f
  data << 0xd215
  data << 0x52ea
  data << 0x4445
  data << 0xa09e
  data << 0x670a
  data << 0xf92c
  data << 0xe8f8

  ## Framing Layer
  # flags and lengths
  data << 0x7258
  # vector E131_DATA_PACKET
  data << 0x0000
  data << 0x0002
  data = data.pack("n*")
  # source name
  name = "test".ljust(64, [0x00].pack("c")).unpack("U*").pack("C*")
  data << name

  # priority
  data << [0x64].pack("c") # default to 100
  # sync universe
  data << [0x0000].pack("n")
  # sequence number and options
  data << [seq_number].pack("c")
  # options
  data << [0x00].pack("c")
  # universe number
  data << [0x0001].pack("n")

  ##DMP Layer
  # flag and size is 1+512+10
  data << [0x720b].pack("n")
  # vector DMP_SET_PROPERTY
  data << [0x02].pack("c")
  # format
  data << [0xa1].pack("c")
  # first property address
  data << [0x0000].pack("n")
  # increment
  data << [0x0001].pack("n")
  # value count
  data << [0x0201].pack("n")

  # start code
  data << [0x00].pack("c")

  512.times do |x|
    data << [rand(255)].pack("c")
  end

  data
end

# require "socket"

# sock = UDPSocket.new

# sock.connect("127.0.0.1", 5568)

# n = 1
# while true
#   puts "sending packet #{n}"
#   sock.send(packet(n), 0)
#   n += 1
#   sleep(0.2)
# end

