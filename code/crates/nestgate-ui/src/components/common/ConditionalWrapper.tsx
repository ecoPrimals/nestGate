import React from 'react';

interface ConditionalWrapperProps {
  condition: boolean;
  wrapper: (children: React.ReactElement) => React.ReactElement;
  children: React.ReactElement;
}

/**
 * A utility component that conditionally wraps children in a wrapper component
 * This is useful for applying conditional layouts, animations, or other wrappers
 */
const ConditionalWrapper: React.FC<ConditionalWrapperProps> = ({ 
  condition, 
  wrapper, 
  children 
}) => {
  return condition ? wrapper(children) : children;
};

export default ConditionalWrapper; 