require "pathname"

require_relative "cue_list"

module Core
  class CueEngine
    attr_reader :current
    
    def initialize(path)
      @path = Pathname.new(path)
      @cuelists = initialize_cuelists
      @current = nil
    end

    def load(identifier)
      unless @cuelists[identifier].nil?
        @current = @cuelists[identifier]
        @current.goto(0)
      end
    end

    def current_file?(file)
      return false if @current.nil?
      @current.cue == file
    end

    def files_to_rerun(file)
      return [] if @current.nil?

      if @current.cue_is_before_current?(file)
        @current.all_cues_till_current
      else
        []
      end
    end

    def reload(identifier)
      if @cuelists[identifier]
        @cuelists[identifier].reload
      end
    end

    private

    def initialize_cuelists
      cuelists = {}

      list_of_directories.each do |path|
        identifier = path.basename.to_s
        cuelists[identifier] = CueList.new(path)
      end

      cuelists
    end

    def list_of_directories
      @path.children.select(&:directory?)
    end
  end

end
