import { useState, useCallback } from 'react';

export interface UseSettingsFormOptions {
  sectionName: string;
  onSubmit?: (values: any) => Promise<void>;
  onSuccess?: (message: string) => void;
  onError?: (message: string) => void;
}

export const useSettingsForm = ({ 
  sectionName, 
  onSubmit, 
  onSuccess, 
  onError 
}: UseSettingsFormOptions) => {
  const [loading, setLoading] = useState(false);

  const handleSave = useCallback(async (values: any) => {
    setLoading(true);
    try {
      if (onSubmit) {
        await onSubmit(values);
      } else {
        // Default behavior - just log the values
        console.log(`${sectionName} settings:`, values);
      }
      
      const successMessage = `${sectionName} settings saved successfully`;
      if (onSuccess) {
        onSuccess(successMessage);
      } else {
        console.log(successMessage);
      }
    } catch (error) {
      console.error(`Error saving ${sectionName} settings:`, error);
      const errorMessage = `Failed to save ${sectionName} settings`;
      if (onError) {
        onError(errorMessage);
      } else {
        console.error(errorMessage);
      }
    } finally {
      setLoading(false);
    }
  }, [sectionName, onSubmit, onSuccess, onError]);

  return {
    loading,
    handleSave,
  };
}; 