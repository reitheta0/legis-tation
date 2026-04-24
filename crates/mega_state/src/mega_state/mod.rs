pub mod base;
pub mod bundle;
pub mod text;
pub mod variants;
///
///                            FORMATTING
///
///
///
///  document covers = docx {
///
///    pieces paragraph table title outline
///
///    config = {
///
///      page_number_location bundle {
///
///        vertical variants {
///
///          upper header
///          
///          lower footer default
///
///        }
///        
///        horizontal variants {
///
///          left
///
///          center default
///
///          right
///
///        }
///
///      } implementation {
///
///        ...
///
///      }
///
///      page_orientation_type variants {
///
///        portrait default
///
///        landscape
///
///      } implementation {
///
///      }
///
///      paper_size variants hashed {
///      
///        A0, [47679, 67407]
///      
///        A1, [33676, 47679]
///      
///        A2, [23811, 33676]
///      
///        A3, [16838, 23811]
///      
///        A4, [11906, 16838] default
///      
///        A5, [8419, 11906]
///      
///        A6, [5953, 8419]
///      
///        A7, [4210, 5953]
///      
///        A8, [2977, 4210]
///      
///        A9, [2105, 2977]
///  
///        Letter, [12240, 15840]
///
///        Legal, [12240, 20160]
///
///      }
///
///    }
///
///  }
///
///  paragraph {
///
///    pieces text
///
///    config = {
///
///      alignment_type
///
///      spacing
///
///      line_spacing_rule
///
///      indent_type bundle {
///
///        left type = i32
///
///        special_indent type = docx_rs::SpecialIndentType
///
///        end type = i32
///
///        start_chars type = i32
///
///      } implementation {
///
///      }
///
///    }
///
///    type = {
///
///      introduction
///
///      body
///
///      closing
///
///    }
///
///  }
///
///  text {
///
///    config = {
///
///      size type = usize default = 12
///
///      color type = #color default = black
///
///      highlight_color type = #color
///
///      shading config = {
///
///        shading_color type = #color
///
///        shading_type
///
///      } implementation = {
///      
///      ...
///
///      }
///
///      font default = Times New Roman
///
///    }
///
///  }
///
///  outline_branch internal {
///  
///    level type = i32
///
///    number type = i32
///
///    content type = String
///
///    formatting type = TextConfig
///
///    bold_formatting type = TextConfig
///
///  }
///
///  nested_array internal variants {
///
///    value {
///
///     T
///
///    }
///
///    Array {
///
///     Vec<NestedArray<T>>
///
///    }
///
///  }
///
///  
pub fn todo() -> String {
  println!("to-do"); //TODO: I JUST WANT TOTO FIX THE Fucking eRRORS
  String::new()
}

/*impl ParagraphState {
  pub fn format_paragraph(&mut self) -> Paragraph {
    let mut resulting_paragraph: Paragraph = self.input_paragraph.clone().unwrap_as_paragraph();
    let formatting = &self.paragraph_formatting;
    match formatting.get_alignment_type() {
      Some(alignment) => {
        resulting_paragraph = resulting_paragraph.align(*alignment);
      }
      None => {}
    }

    match formatting.get_spacing() {
      Some(spacing) => {
        resulting_paragraph = resulting_paragraph.line_spacing(
          LineSpacing::new().line((spacing * 240.0) as i32).line_rule(
            self
              .paragraph_formatting
              .get_line_spacing_rule()
              .unwrap_or(LineSpacingType::Auto),
          ),
        )
      }
      None => {}
    }

    match &formatting.get_indent_type() {
      Some(indentation) => {
        resulting_paragraph = resulting_paragraph.indent(
          *indentation.get_left(),
          *indentation.get_special_indent(),
          *indentation.get_end(),
          *indentation.get_start_chars(),
        )
      }
      None => {}
    }

    resulting_paragraph
  }

  pub fn paragraph_from_string(
    input_string: String,
    text_formatting: &TextConfig,
    paragraph_formatting: ParagraphConfig,
  ) -> Paragraph {
    ParagraphState::new(
      ParagraphData::VecParagraph(vec![TextState::new(input_string, text_formatting.clone())]),
      paragraph_formatting,
    )
    .format_paragraph()
  }
}*/
