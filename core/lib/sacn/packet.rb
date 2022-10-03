module SACN
  PREAMBLE_SIZE = [0x0010].pack("n")
  POSTAMBLE_SIZE = [0x0000].pack("n")
  ACN_IDENTIFIER = [0x41, 0x53, 0x43, 0x2d, 0x45, 0x31, 0x2e, 0x31, 0x37, 0x00, 0x00, 0x00].pack("c*")
  CID_UUID = [0xe68f, 0xd215, 0x52ea, 0x4445, 0xa09e, 0x670a, 0xf92c, 0xe8f8].pack("n*")
  # TODO: This isn't really a constant
  SYNC_UNIVERSE = [0x0000].pack("n")
  FORMAT_IDENTIFIER = [0xa1].pack("c")
  FIRST_PROPERTY_ADDRESS = [0x0000].pack("n")
  ADDRESS_INCREMENT = [0x0001].pack("n")
  DMX_START_CODE = [0x00].pack("c")

  VECTOR_ROOT_E131_DATA = [0x0000, 0x0004].pack("nn")
  VECTOR_E131_DATA_PACKET = [0x0000, 0x0002].pack("nn")
  VECTOR_DMP_SET_PROPERTY = [0x02].pack("c")

  HEADER_SIZE = 16
  ROOT_LAYER_SIZE = 38
  FRAMING_LAYER_SIZE = 77
  DMP_LAYER_SIZE = 10

  class Packet
    def initialize(source)
      @source = source
      @data = ""
    end

    protected
    

  end

  class DataPacket
    def initialize(source, dmx_data, universe)
      @data = ""
      @source = source
      @dmx_data = dmx_data
      @universe = universe
    end

    def data(sequence_number)
      @data = ""

      root_layer
      framing_layer(sequence_number)
      dmp_layer

      @data
    end

    private

    def root_layer
      @data << PREAMBLE_SIZE
      @data << POSTAMBLE_SIZE
      @data << ACN_IDENTIFIER
      @data << flag_root_length
      @data << VECTOR_ROOT_E131_DATA
      @data << CID_UUID
    end

    def framing_layer(sequence_number)
      @data << flag_framing_length
      @data << VECTOR_E131_DATA_PACKET
      @data << @source.name
      @data << [@source.priority].pack("c")
      @data << SYNC_UNIVERSE
      @data << [sequence_number].pack("c")
      @data << [0x00].pack("c")
      @data << [@universe].pack("n")
    end

    def dmp_layer
      @data << flag_dmp_length
      @data << VECTOR_DMP_SET_PROPERTY
      @data << FORMAT_IDENTIFIER
      @data << FIRST_PROPERTY_ADDRESS
      @data << ADDRESS_INCREMENT
      @data << [data_size].pack("n")
      @data << DMX_START_CODE
      @data << @dmx_data.pack("c*")
    end
    
    def flag_root_length
      size = total_size - HEADER_SIZE
      [0x7 << 12 | size].pack("n")
    end

    def flag_framing_length
      size = total_size - ROOT_LAYER_SIZE
      [0x7 << 12 | size].pack("n")
    end

    def flag_dmp_length
      size = total_size - ROOT_LAYER_SIZE - FRAMING_LAYER_SIZE
      [0x7 << 12 | size].pack("n")
    end

    def total_size
      data_size + ROOT_LAYER_SIZE + FRAMING_LAYER_SIZE + DMP_LAYER_SIZE
    end

    def data_size
      @dmx_data.length + 1
    end
  end
end
