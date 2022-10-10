module Core
  class CueList
    def initialize(path)
      @path = path
      @cues = cue_paths
      @current = 0
    end

    def go
      @current += 1
      @current = 0 if @current >= @cues.length
    end

    # TODO: Cues should be able to named as well as numbered, so
    # we have to be more careful how we check for cues existance
    def goto(n)
      return [] if n >= @cues.length || @current == n


      old = @current
      @current = n

      return [true, [cue]] if @current == 0

      if n > old
        [false, files_to_run = @cues[old..n]]
      elsif n < old
        [true, @cues[..@current]]
      end
    end

    def cue
      @cues[@current]
    end

    def cue_is_before_current?(target_cue)
      index = @cues.find_index { |cue| cue.to_s == target_cue }
      return false if index.nil?

      index <= @current
    end

    def all_cues_till_current
      @cues[..@current]
    end

    def reload
      current_cue_path = @cues[@current]

      @cues = cue_paths
      @current = @cues.find_index { |cue| cue == current_cue_path }
    end

    def to_s
      "CueList '#{@path.basename.to_s}' - Total: #{@cues.length} - Active: #{cue.basename}"
    end

    private

    def cue_paths
      @path.children.sort.map { |child| child.expand_path }
    end
  end
end
